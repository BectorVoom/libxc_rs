//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 627/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk627<F: Float>(t25036: F, t25042: F, t25146: F, t25154: F, t25163: F, t28811: F, t28814: F, t28819: F, t28824: F, t28829: F, t28833: F, t28838: F, t28885: F, t28897: F, t28911: F, t871: F) -> (F, F) {
    let t28922 = -t25036 - t25042 / 9.0 + t25146 / 6.0 - t25154 - t28811 / 3.0 - 2.0 / 3.0 * t28814 + t28819 / 4.0 + t28824 / 4.0 - t25163 / 18.0 + 2.0 * t28829 + 2.0 * t28833 + 2.0 * t28838;
    let t28924 = t28885 + t28897 + t28911 + t28922;
    let t28925 = t871 * t28924;
    (t28924, t28925)
}

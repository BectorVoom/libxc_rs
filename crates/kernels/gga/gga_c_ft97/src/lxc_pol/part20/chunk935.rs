//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 935/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk935<F: Float>(t28835: F, t824: F, t193: F, t89: F, t25042: F, t25146: F, t25163: F, t25343: F, t25351: F, t28811: F, t28814: F, t28819: F, t28824: F, t28829: F, t28833: F, t28538: F, t28767: F, t28807: F) -> (F, F, F) {
    let t28836 = t28835 * t824;
    let t28837 = t193 * t28836;
    let t28838 = t89 * t28837;
    let t28840 = -t25343 - t25042 / 27.0 + t25146 / 18.0 - t25351 - t28811 / 9.0 - 2.0 / 9.0 * t28814 + t28819 / 12.0 + t28824 / 12.0 - t25163 / 54.0 + 2.0 / 3.0 * t28829 + 2.0 / 3.0 * t28833 + 2.0 / 3.0 * t28838;
    let t28842 = t28538 + t28767 + t28807 + t28840;
    (t28836, t28838, t28842)
}

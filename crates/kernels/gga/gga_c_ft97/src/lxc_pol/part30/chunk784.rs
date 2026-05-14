//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 784/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk784<F: Float>(t193: F, t35993: F, t89: F, t35972: F, t799: F, t27: F, t33867: F, t33960: F, t33977: F, t35861: F, t35866: F, t35975: F, t35979: F, t35983: F, t35987: F, t35991: F) -> (F, F, F, F) {
    let t35994 = t193 * t35993;
    let t35995 = t89 * t35994;
    let t35997 = t799 * t35972;
    let t35999 = t89 * t27 * t35997;
    let t36001 = t33867 + t35861 / 18.0 + t35866 / 3.0 - t35975 / 6.0 - t33960 - 2.0 / 9.0 * t35979 - 2.0 * t35983 + 4.0 / 3.0 * t35987 + t33977 + t35991 / 9.0 + 2.0 / 3.0 * t35995 - t35999 / 3.0;
    (t35995, t35997, t35999, t36001)
}

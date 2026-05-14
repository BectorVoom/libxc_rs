//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 827/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk827<F: Float>(t1403: F, t33254: F, t681: F, t33574: F, t33590: F, t5996: F, t2: F, t33452: F, t231: F, t33301: F, t33300: F, t898: F, t1424: F, t7242: F, t24447: F, t631: F) -> (F, F, F, F, F, F, F, F) {
    let t140707 = t1403 * t681 * t33254;
    let t140710 = t1403 * t681 * t33574;
    let t140712 = t5996 * t33590;
    let t140714 = t2 * t33452;
    let t140744 = t231 * t33301;
    let t140756 = t898 * t33300;
    let t140757 = t7242 * t1424;
    let t140762 = t24447 * t631;
    (t140707, t140710, t140712, t140714, t140744, t140756, t140757, t140762)
}

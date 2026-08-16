//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 538/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk538<F: Float>(t263: F, t6061: F, t1424: F, t771: F, t1410: F, t2426: F, t3771: F, t5567: F, t6041: F, t1614: F, t209: F, t9: F) -> (F, F, F, F, F, F) {
    let t24240 = t6061 * t263;
    let t24245 = t1424 * t771;
    let t24260 = t2426 * t1410;
    let t24265 = t3771 * t6041 * t5567;
    let t24274 = t1614 * t209;
    let t24275 = t9 * t24274;
    (t24240, t24245, t24260, t24265, t24274, t24275)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 559/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk559<F: Float>(t9845: F, t969: F, t9829: F, t825: F, t2465: F, t2571: F, t2464: F, t313: F, t9725: F, t9739: F, t1645: F, t9740: F) -> (F, F, F, F, F, F) {
    let t9846 = F::new(0.38342925953920749676e0) * t9845;
    let t9847 = t969 * t9829;
    let t9848 = t825 * t9847;
    let t9849 = F::new(0.38342925953920749676e0) * t9848;
    let t9850 = t2465 * t2571;
    let t9851 = t2464 * t9850;
    let t9852 = t825 * t9851;
    let t9853 = F::new(0.85206502119823888169e-1) * t9852;
    let t9854 = t313 * t9725;
    let t9857 = t313 * t9739;
    let t9858 = t1645 * t9740;
    (t9846, t9849, t9853, t9854, t9857, t9858)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 558/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk558<F: Float>(t2685: F, t9829: F, t2684: F, t2465: F, t2581: F, t2464: F, t3311: F, t7416: F, t549: F, t9744: F, t9748: F, t2013: F, t3296: F) -> (F, F, F, F, F, F) {
    let t9830 = t2685 * t9829;
    let t9831 = t2684 * t9830;
    let t9832 = F::cast_from(0.38342925953920749676e0_f64) * t9831;
    let t9833 = t2465 * t2581;
    let t9834 = t2464 * t9833;
    let t9835 = t2684 * t9834;
    let t9836 = F::cast_from(0.85206502119823888169e-1_f64) * t9835;
    let t9837 = t7416 * t3311;
    let t9838 = F::cast_from(0.38342925953920749676e0_f64) * t9837;
    let t9839 = t549 * t9744;
    let t9842 = t549 * t9748;
    let t9845 = t2013 * t3296;
    (t9832, t9836, t9838, t9839, t9842, t9845)
}

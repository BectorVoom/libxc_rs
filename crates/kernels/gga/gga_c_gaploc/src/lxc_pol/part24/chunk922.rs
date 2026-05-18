//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 922/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk922<F: Float>(t9837: F, t2013: F, t3296: F, t969: F, t9829: F, t825: F, t2465: F, t2571: F, t2464: F, t2194: F, t3308: F, t7068: F, t883: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9838 = F::new(0.38342925953920749676e0) * t9837;
    let t9845 = t2013 * t3296;
    let t9846 = F::new(0.38342925953920749676e0) * t9845;
    let t9847 = t969 * t9829;
    let t9848 = t825 * t9847;
    let t9849 = F::new(0.38342925953920749676e0) * t9848;
    let t9850 = t2465 * t2571;
    let t9851 = t2464 * t9850;
    let t9852 = t825 * t9851;
    let t9853 = F::new(0.85206502119823888169e-1) * t9852;
    let t9873 = t2194 * t3308;
    let t9889 = t883 * t7068;
    (t9838, t9846, t9847, t9849, t9850, t9851, t9853, t9873, t9889)
}

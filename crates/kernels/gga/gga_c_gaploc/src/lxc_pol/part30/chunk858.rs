//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 858/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk858<F: Float>(t9848: F, t2465: F, t2571: F, t2464: F, t825: F, t2194: F, t3308: F, t7068: F, t883: F, t1967: F, t7810: F, t7403: F, t959: F, t7340: F, t3281: F, t5676: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9849 = 0.38342925953920749676e0 * t9848;
    let t9850 = t2465 * t2571;
    let t9851 = t2464 * t9850;
    let t9852 = t825 * t9851;
    let t9853 = 0.85206502119823888169e-1 * t9852;
    let t9873 = t2194 * t3308;
    let t9889 = t883 * t7068;
    let t9890 = t1967 * t9889;
    let t9891 = t7810 * t9890;
    let t9892 = 0.38342925953920749676e0 * t9891;
    let t9935 = 0.29792074959875355558e-1 * t7403 * t959;
    let t9937 = 0.29792074959875355558e-1 * t7340 * t959;
    let t9942 = 0.29792074959875355558e-1 * t5676 * t3281;
    (t9849, t9850, t9851, t9853, t9873, t9890, t9892, t9935, t9937, t9942)
}

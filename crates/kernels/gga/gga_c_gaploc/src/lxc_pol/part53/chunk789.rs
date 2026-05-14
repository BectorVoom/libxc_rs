//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 789/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk789<F: Float>(t326: F, t43508: F, t825: F, t2684: F, t7585: F, t33360: F, t787: F, t9824: F, t33348: F, t13042: F, t2197: F, t8793: F, t9950: F, t3040: F, t41236: F, t1022: F, t9755: F) -> (F, F, F, F, F, F, F, F) {
    let t43511 = 0.92023022289409799224e1 * t825 * t326 * t43508;
    let t43514 = 0.43710935587469654631e2 * t2684 * t7585 * t43508;
    let t43522 = t787 * t33360 * t9824;
    let t43523 = 0.29792074959875355558e-1 * t43522;
    let t43526 = t787 * t33348 * t9824;
    let t43527 = 0.29792074959875355558e-1 * t43526;
    let t43567 = 0.43710935587469654631e2 * t2197 * t13042;
    let t43569 = 0.10725146985555128001e1 * t8793 * t9950;
    let t43571 = 0.35750489951850426669e0 * t41236 * t3040;
    let t43572 = t9755 * t1022;
    (t43511, t43514, t43523, t43527, t43567, t43569, t43571, t43572)
}

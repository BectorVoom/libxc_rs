//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2681/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2681(t118: f64, t20416: f64, t3739: f64, t794: f64, t16094: f64, t16095: f64, t6347: f64, t686: f64, t213: f64, t1307: f64, t16084: f64, t16101: f64, t19631: f64, t19781: f64, t20356: f64, t221: f64, t40351: f64, t5187: f64, t5195: f64, t5196: f64, t54728: f64, t56482: f64, t56484: f64, t56491: f64, t56493: f64) -> f64 {
    let t74702 = t3739 * t118 * t794 * t20416;
    let t74724 = t16094 * t686 * t16095 * t6347;
    let t74726 = t213 * t20416;
    let t74735 = 0.8333333333333333333e-3_f64 * t74702 - 0.19999999999999999999e-1_f64 * t40351 + 0.99999999999999999995e-1_f64 * t54728 * t221 * t213 * t20356 * t1307 - 0.59999999999999999997e-1_f64 * t16101 * t221 * t19781 * t5187 + 0.14999999999999999999e-1_f64 * t5195 * t221 * t16084 * t6347 + 0.14999999999999999999e-1_f64 * t5195 * t221 * t5196 * t19631 - 0.74999999999999999995e-2_f64 * t74724 + 0.49999999999999999998e-2_f64 * t5195 * t221 * t74726 * t1307 + 0.24999999999999999999e-2_f64 * t56482 + 0.11666666666666666666e0_f64 * t56484 - 0.38888888888888888887e-1_f64 * t56491 - 0.34999999999999999998e-1_f64 * t56493;
    t74735
}

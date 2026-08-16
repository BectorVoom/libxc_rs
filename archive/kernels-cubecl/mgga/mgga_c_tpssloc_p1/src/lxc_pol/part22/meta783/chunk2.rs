//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2681/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2681<F: Float>(t118: F, t20416: F, t3739: F, t794: F, t16094: F, t16095: F, t6347: F, t686: F, t213: F, t1307: F, t16084: F, t16101: F, t19631: F, t19781: F, t20356: F, t221: F, t40351: F, t5187: F, t5195: F, t5196: F, t54728: F, t56482: F, t56484: F, t56491: F, t56493: F) -> F {
    let t74702 = t3739 * t118 * t794 * t20416;
    let t74724 = t16094 * t686 * t16095 * t6347;
    let t74726 = t213 * t20416;
    let t74735 = F::cast_from(0.8333333333333333333e-3_f64) * t74702 - F::cast_from(0.19999999999999999999e-1_f64) * t40351 + F::cast_from(0.99999999999999999995e-1_f64) * t54728 * t221 * t213 * t20356 * t1307 - F::cast_from(0.59999999999999999997e-1_f64) * t16101 * t221 * t19781 * t5187 + F::cast_from(0.14999999999999999999e-1_f64) * t5195 * t221 * t16084 * t6347 + F::cast_from(0.14999999999999999999e-1_f64) * t5195 * t221 * t5196 * t19631 - F::cast_from(0.74999999999999999995e-2_f64) * t74724 + F::cast_from(0.49999999999999999998e-2_f64) * t5195 * t221 * t74726 * t1307 + F::cast_from(0.24999999999999999999e-2_f64) * t56482 + F::cast_from(0.11666666666666666666e0_f64) * t56484 - F::cast_from(0.38888888888888888887e-1_f64) * t56491 - F::cast_from(0.34999999999999999998e-1_f64) * t56493;
    t74735
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1199/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1199<F: Float>(t3006: F, t5253: F, t3034: F, t4758: F, t969: F, t3025: F, t1692: F, t9634: F, t1211: F, t5208: F, t10862: F, t10874: F, t10877: F, t10884: F, t10960: F, t1221: F, t15328: F, t15331: F, t15335: F, t15342: F, t15345: F, t3570: F, t3575: F, t3585: F, t3592: F, t5211: F, t5247: F, t5254: F) -> F {
    let t15348 = t5253 * t3006;
    let t15351 = t4758 * t3034;
    let t15352 = t15351 * t969;
    let t15355 = t5253 * t3025;
    let t15358 = t1692 * t9634;
    let t15359 = t15358 * t3006;
    let t15362 = t5208 * t1211;
    let t15367 = F::cast_from(0.64329366355741395948e2_f64) * t3575 * t15328 + F::cast_from(0.32164683177870697974e2_f64) * t3575 * t15331 + F::cast_from(0.20691336878655965246e4_f64) * t10862 * t15335 - F::cast_from(0.23392893589820816284e1_f64) * t10960 * t5247 + F::cast_from(0.34631511798751726598e2_f64) * t10874 * t5254 - F::cast_from(0.23392893589820816284e1_f64) * t3585 * t15342 - F::cast_from(0.11696446794910408142e1_f64) * t3585 * t15345 - F::cast_from(0.1038945353962551798e3_f64) * t10877 * t15348 + F::cast_from(0.34631511798751726598e2_f64) * t3592 * t15352 + F::cast_from(0.17315755899375863299e2_f64) * t3592 * t15355 + F::cast_from(0.1025389702100779493e4_f64) * t10884 * t15359 + F::cast_from(2.0_f64) * t15362 * t1221 + F::cast_from(1.0_f64) * t5211 * t3570;
    t15367
}

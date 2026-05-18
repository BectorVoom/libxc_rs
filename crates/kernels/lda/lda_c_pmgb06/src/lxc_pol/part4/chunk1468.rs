//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1468/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1468<F: Float>(t6961: F, t707: F, t6957: F, t10609: F, t10614: F, t10617: F, t10623: F, t10635: F, t10640: F, t10643: F, t10646: F, t10657: F, t10661: F, t14587: F, t14623: F, t14639: F, t14642: F, t14646: F, t18785: F, t18876: F, t346: F, t387: F, t5583: F, t73: F) -> F {
    let t18883 = t707 * t6961;
    let t18885 = t707 * t6957;
    let t18892 = F::new(0.008135887625008338) * t10609 - t10614 + t10617 - F::new(0.0005811348303577384) * t10623 - F::new(0.0017434044910732151) * t10635 + t10640 - t10643 - F::new(0.013430671634934398) * t10646 - F::new(0.10809180959278285) * t14623 + t346 * (t18785 + t18876) * t387 * t73 + F::new(12.0) * t5583 * t14587 + F::new(0.039914113367515366) * t18883 + F::new(0.039914113367515366) * t18885 + F::new(0.19816831758676853) * t10657 - F::new(1.849570964143173) * t10661 - F::new(0.0023245393214309535) * t14639 - F::new(0.0017434044910732151) * t14642 - F::new(0.0005811348303577384) * t14646;
    t18892
}

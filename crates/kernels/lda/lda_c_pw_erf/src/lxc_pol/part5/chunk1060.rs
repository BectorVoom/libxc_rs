//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1060/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1060<F: Float>(t142: F, t774: F, t1549: F, t6093: F, t1: F, t1750: F, t1755: F, t2686: F, t11299: F, t2706: F, t2741: F, t4387: F, t4389: F, t4391: F, t4398: F, t4406: F, t4408: F, t4412: F, t5968: F, t7328: F, t7329: F, t8097: F, t8098: F, t8099: F, t8101: F, t8102: F, t8103: F) -> (F, F, F, F) {
    let t19866 = t774 * t142;
    let t19872 = t1549 * t6093;
    let t19882 = t2686 * t1750 * t1 * t1755;
    let t19961 = -t8097 - F::new(3.5089340384731225) * t5968 + t8098 + t8099 + t2706 - t8101 - t8102 + F::new(0.0021973866044103793) * t4387 - F::new(5.263401057709683) * t4389 - F::new(155.84180309438278) * t4391 - t11299 - t8103 - t2741 + F::new(0.09759222794503372) * t4398 - t7328 + t7329 + F::new(9.0) * t4406 + F::new(180.0) * t4408 + F::new(10.526802115419367) * t4412;
    (t19866, t19872, t19882, t19961)
}

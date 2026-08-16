//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1060/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1060(t142: f64, t774: f64, t1549: f64, t6093: f64, t1: f64, t1750: f64, t1755: f64, t2686: f64, t11299: f64, t2706: f64, t2741: f64, t4387: f64, t4389: f64, t4391: f64, t4398: f64, t4406: f64, t4408: f64, t4412: f64, t5968: f64, t7328: f64, t7329: f64, t8097: f64, t8098: f64, t8099: f64, t8101: f64, t8102: f64, t8103: f64) -> (f64, f64, f64, f64) {
    let t19866 = t774 * t142;
    let t19872 = t1549 * t6093;
    let t19882 = t2686 * t1750 * t1 * t1755;
    let t19961 = -t8097 - 3.5089340384731225_f64 * t5968 + t8098 + t8099 + t2706 - t8101 - t8102 + 0.0021973866044103793_f64 * t4387 - 5.263401057709683_f64 * t4389 - 155.84180309438278_f64 * t4391 - t11299 - t8103 - t2741 + 0.09759222794503372_f64 * t4398 - t7328 + t7329 + 9.0_f64 * t4406 + 180.0_f64 * t4408 + 10.526802115419367_f64 * t4412;
    (t19866, t19872, t19882, t19961)
}

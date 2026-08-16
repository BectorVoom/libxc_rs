//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1379/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1379(t10632: f64, t10811: f64, t1569: f64, t1581: f64, t17428: f64, t21115: f64, t21195: f64, t2861: f64, t2888: f64, t41826: f64, t4411: f64, t49430: f64, t5743: f64, t5759: f64, t5762: f64, t59920: f64, t60407: f64, t69047: f64, t69182: f64, t76637: f64, t76647: f64, t76652: f64, t76654: f64, t76657: f64, t76659: f64, t76661: f64, t77220: f64, t77239: f64, t77328: f64, t932: f64, t943: f64, t951: f64) -> f64 {
    let t77370 = -6.0_f64 * t2861 * t77328 * t932 - 0.12304822629859687989e5_f64 * t41826 * t76637 * t10632 + 0.5848223622634646207e0_f64 * t943 * t77220 * t951 - t76647 + 6.0_f64 * t17428 * t5759 + 0.1929837539843104208e3_f64 * t60407 * t5762 + 4.0_f64 * t4411 * t21195 + 4.0_f64 * t69182 * t1569 + t76652 + t76654 - t76657 - 12.0_f64 * t59920 * t5743 - 0.77193501593724168322e3_f64 * t49430 * t21115 + 0.11579025239058625248e4_f64 * t10811 * t77239 * t2888 + 0.23392894490538584828e1_f64 * t69047 * t1581 - t76659 - t76661;
    t77370
}

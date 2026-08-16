//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1983/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1983(t11665: f64, t1218: f64, t1232: f64, t15470: f64, t15474: f64, t15478: f64, t15484: f64, t15488: f64, t15490: f64, t15494: f64, t15495: f64, t15498: f64, t15503: f64, t15507: f64, t3511: f64, t3518: f64, t3527: f64, t3577: f64, t3587: f64, t4954: f64, t5005: f64, t5024: f64) -> f64 {
    let t15512 = -t11665 * t4954 / 2304.0_f64 - t3577 * t15470 / 2304.0_f64 - t3577 * t15474 / 4608.0_f64 - t3577 * t15478 / 2304.0_f64 + 5.0_f64 / 13824.0_f64 * t5005 * t3587 - t15484 - t15488 + t15490 + t15494 - t15495 * t1218 / 288.0_f64 + t15498 * t1232 / 432.0_f64 - t15503 * t3511 / 288.0_f64 + t15507 * t3518 / 576.0_f64 + t5024 * t3527 / 864.0_f64;
    t15512
}

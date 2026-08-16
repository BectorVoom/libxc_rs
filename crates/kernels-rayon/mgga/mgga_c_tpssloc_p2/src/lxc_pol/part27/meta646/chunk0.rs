//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2219/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2219(t14192: f64, t6717: f64, t13965: f64, t6755: f64, t25577: f64, t3103: f64, t25650: f64, t3030: f64, t82890: f64, t1618: f64, t23422: f64, t23433: f64, t23489: f64, t23544: f64, t25652: f64, t25654: f64, t25655: f64, t25679: f64, t3123: f64, t3128: f64, t4585: f64, t4609: f64, t4649: f64, t4652: f64, t7583: f64, t82981: f64, t83068: f64, t83127: f64) -> (f64, f64) {
    let t88636 = t6717 * t14192 / 432.0_f64;
    let t88645 = t6755 * t13965;
    let t88648 = t25577 * t3103 / 1152.0_f64;
    let t88655 = t25650 * t82890 * t3030;
    let t88662 = -t23422 * t4609 / 54.0_f64 + t88636 - t23544 * t4585 / 576.0_f64 + 0.10093189023535097714e-3_f64 * t82981 * t7583 + 0.20186378047070195428e-3_f64 * t23489 * t25679 + t25577 * t3123 / 1536.0_f64 - t88645 / 6912.0_f64 + t88648 + t83068 * t1618 / 1536.0_f64 + t23433 * t4652 / 768.0_f64 + 0.72670960969452703541e-2_f64 * t83127 + 0.40372756094140390856e-3_f64 * t88655 * t25655 + 0.40372756094140390856e-3_f64 * t25652 * t3128 * t4649 * t25654;
    (t88655, t88662)
}

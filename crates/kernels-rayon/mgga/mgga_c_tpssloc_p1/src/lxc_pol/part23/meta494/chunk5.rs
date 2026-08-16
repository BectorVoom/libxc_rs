//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1524/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1524(t1268: f64, t1458: f64, t1774: f64, t1849: f64, t19451: f64, t20293: f64, t20296: f64, t20347: f64, t20350: f64, t20720: f64, t22425: f64, t28002: f64, t4028: f64, t510: f64, t513: f64, t5460: f64, t5493: f64, t574: f64, t6287: f64, t6295: f64, t6468: f64, t652: f64, t67001: f64, t7458: f64, t7676: f64, t79713: f64, t79817: f64, t79825: f64, t79829: f64, t79855: f64, t79891: f64, t79903: f64, t79915: f64, t79926: f64, t79939: f64, t79988: f64, t80534: f64, t88: f64, t89: f64) -> f64 {
    let t80558 = -8.0_f64 * t652 * t22425 * t1458 - 12.0_f64 * t652 * t6287 * t5493 - 8.0_f64 * t7458 * t20720 - 8.0_f64 * t4028 * t20720 + 6.0_f64 * t6295 * t6468 - 2.0_f64 * t652 * t510 * t79817 - 24.0_f64 * t19451 * t5460 - 4.0_f64 * t20293 * t1774 - 6.0_f64 * t89 * t79825 * t510 - 12.0_f64 * t79829 * t510 - 24.0_f64 * t20296 * t1774 + t513 * (t79855 + t79891 + t79903 + t79915 + t79926 + t79939 + t79988 + t80534) + 4.0_f64 * t20350 * t1849 + (2.0_f64 * t1268 * t79817 + 8.0_f64 * t1458 * t67001 + 12.0_f64 * t19451 * t5493 + 8.0_f64 * t20347 * t4028 + 8.0_f64 * t20347 * t7676 + 24.0_f64 * t28002 * t5493 + 6.0_f64 * t79825 * t88 + t79713 + 12.0_f64 * t79829) * t574;
    t80558
}

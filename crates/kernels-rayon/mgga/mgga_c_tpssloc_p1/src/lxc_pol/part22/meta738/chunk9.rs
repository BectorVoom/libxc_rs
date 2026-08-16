//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2431/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2431(t10740: f64, t10747: f64, t10771: f64, t10825: f64, t1569: f64, t1580: f64, t1581: f64, t17297: f64, t17349: f64, t17454: f64, t17493: f64, t17544: f64, t21309: f64, t21312: f64, t21321: f64, t2861: f64, t2905: f64, t2930: f64, t4434: f64, t4438: f64, t4472: f64, t48783: f64, t49263: f64, t49422: f64, t5758: f64, t5790: f64, t59895: f64, t69011: f64, t69018: f64, t69036: f64) -> f64 {
    let t69249 = -0.35089341735807877242e1_f64 * t10747 * t21309 - 0.35089341735807877242e1_f64 * t2905 * t4472 * t5790 - 0.35089341735807877242e1_f64 * t2905 * t1581 * t17297 + 0.51947577317044391277e2_f64 * t10825 * t21312 + 0.51947577317044391277e2_f64 * t2930 * t59895 * t1580 + t69011 + 0.10526802520742363173e2_f64 * t48783 * t17454 - t69018 - 0.57895126195293126241e3_f64 * t10771 * t5758 * t4438 + 18.0_f64 * t49422 * t17544 - t69036 - 0.31168546390226634766e3_f64 * t49263 * t17493 - 6.0_f64 * t10740 * t21321 - 6.0_f64 * t2861 * t4434 * t5758 - 6.0_f64 * t2861 * t1569 * t17349;
    t69249
}

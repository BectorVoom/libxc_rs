//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1347/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1347(t10702: f64, t2844: f64, t76644: f64, t1557: f64, t68924: f64, t17195: f64, t5727: f64, t5730: f64, t59959: f64, t21300: f64, t4354: f64, t1637: f64, t4700: f64, t68711: f64, t76634: f64, t76636: f64, t76641: f64, t76643: f64, t76647: f64, t76652: f64, t76654: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t76657 = 0.57895126195293126241e3_f64 * t10702 * t76644 * t2844;
    let t76659 = 4.0_f64 * t68924 * t1557;
    let t76661 = 6.0_f64 * t17195 * t5727;
    let t76663 = 0.96491876992155210402e2_f64 * t59959 * t5730;
    let t76665 = 4.0_f64 * t4354 * t21300;
    let t76666 = -4.0_f64 * t1637 * t4700 * t68711 + t76634 - t76636 - t76641 + t76643 + t76647 - t76652 - t76654 + t76657 + t76659 + t76661 + t76663 + t76665;
    (t76657, t76659, t76661, t76663, t76665, t76666)
}

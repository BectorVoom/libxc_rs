//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 851/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk851(t2743: f64, t5326: f64, t1419: f64, t959: f64, t1422: f64, t2483: f64, t725: f64, t41: f64, t2794: f64, t410: f64, t406: f64, t5331: f64, t5335: f64, t5336: f64, t5338: f64, t5340: f64) -> f64 {
    let t7699 = t2743 * t5326;
    let t7701 = t1419 * t959;
    let t7703 = t1422 * t959;
    let t7705 = t2483 * t725;
    let t7707 = 2.0_f64 * t41 * t7705;
    let t7708 = t410 * t2794;
    let t7710 = t406 * t2794;
    let t7715 = 0.1350520664e0_f64 * t7699 - 12.0_f64 * t7701 + 32.0_f64 * t7703 - t7707 + 8.0_f64 * t7708 - 8.0_f64 * t7710 - t5331 + t5335 + 0.17315859105681463759e2_f64 * t5336 - 0.46785788981077169656e1_f64 * t5338 + 0.69263436422725855036e2_f64 * t5340;
    t7715
}

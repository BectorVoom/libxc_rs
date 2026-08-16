//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1261/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1261(t1639: f64, t520: f64, t6419: f64, t5745: f64, t1838: f64, t5407: f64, t21834: f64, t1773: f64, t21804: f64, t522: f64, t1657: f64, t1772: f64, t1842: f64, t18496: f64, t19509: f64, t20157: f64, t21061: f64, t21805: f64, t21820: f64, t21823: f64, t21827: f64, t21831: f64, t21836: f64, t538: f64, t5433: f64, t5449: f64, t5739: f64, t5921: f64, t6260: f64, t6425: f64, t6430: f64, t6433: f64) -> (f64, f64, f64, f64, f64) {
    let t21840 = t6419 * t1639 * t520;
    let t21841 = t5745 * t21840;
    let t21846 = t5745 * t1838 * t5407 * t520;
    let t21849 = t5745 * t21834 * t520;
    let t21852 = t1773 * t522 * t21804;
    let t21854 = -2.0_f64 * t1657 * t20157 - t1772 * t21852 - t1842 * t21061 - 4.0_f64 * t18496 * t21823 + 4.0_f64 * t19509 * t6425 + 2.0_f64 * t19509 * t6430 + t21805 * t538 - 6.0_f64 * t21820 * t5739 + 4.0_f64 * t21827 * t5739 + 2.0_f64 * t21831 * t5739 - 2.0_f64 * t21836 * t5739 + 2.0_f64 * t21841 * t5739 + t21846 * t5739 + t21849 * t5739 + 2.0_f64 * t5433 * t5921 - t5449 * t5921 - 2.0_f64 * t6260 * t6433;
    (t21841, t21846, t21849, t21852, t21854)
}

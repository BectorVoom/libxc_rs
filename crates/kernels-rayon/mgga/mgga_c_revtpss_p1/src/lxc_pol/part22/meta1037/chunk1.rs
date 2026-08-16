//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3627/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3627(t20394: f64, t3531: f64, t20896: f64, t12571: f64, t6556: f64, t1196: f64, t20890: f64, t43977: f64, t68631: f64, t68633: f64, t68636: f64, t68640: f64, t68683: f64, t68686: f64, t68689: f64, t68692: f64, t68694: f64) -> (f64, f64, f64, f64, f64) {
    let t68696 = 0.46785788981077169656e1_f64 * t3531 * t20394;
    let t68698 = 0.34631718211362927518e2_f64 * t3531 * t20896;
    let t68700 = 0.17315859105681463759e2_f64 * t12571 * t6556;
    let t68703 = 0.10254018858216406658e4_f64 * t1196 * t20890 * t43977;
    let t68704 = t68631 + t68633 + t68636 + t68640 - t68683 - t68686 - t68689 - t68692 - t68694 + t68696 - t68698 - t68700 - t68703;
    (t68696, t68698, t68700, t68703, t68704)
}

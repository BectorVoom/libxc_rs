//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2610/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2610(t3495: f64, t6534: f64, t1189: f64, t1196: f64, t12552: f64, t6518: f64, t1187: f64, t12555: f64, t3520: f64, t5206: f64, t20571: f64, t20573: f64, t20576: f64, t20579: f64, t20582: f64, t20631: f64, t20633: f64, t20635: f64, t20637: f64, t20639: f64, t20643: f64, t20647: f64, t20650: f64, t20654: f64, t20690: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20886 = t3495 * t6534;
    let t20887 = t20886 * t1189;
    let t20889 = 0.11696447245269292414e1_f64 * t1196 * t20887;
    let t20890 = t12552 * t6518;
    let t20891 = t12555 * t1187;
    let t20892 = t20890 * t20891;
    let t20894 = 0.10254018858216406658e4_f64 * t1196 * t20892;
    let t20895 = t3520 * t6534;
    let t20896 = t20895 * t5206;
    let t20898 = 0.17315859105681463759e2_f64 * t1196 * t20896;
    let t20899 = t20690 + t20889 - t20894 - t20898 - t20571 + t20573 + t20576 - t20579 - t20582 + t20631 + t20633 + t20635 - t20637 + t20639 - t20643 + t20647 + t20650 + t20654;
    (t20886, t20887, t20889, t20890, t20892, t20894, t20895, t20896, t20898, t20899)
}

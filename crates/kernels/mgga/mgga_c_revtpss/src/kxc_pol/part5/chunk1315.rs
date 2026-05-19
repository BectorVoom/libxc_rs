//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1315/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1315<F: Float>(t3495: F, t6534: F, t1189: F, t1196: F, t12552: F, t6518: F, t1187: F, t12555: F, t3520: F, t5206: F, t20571: F, t20573: F, t20576: F, t20579: F, t20582: F, t20631: F, t20633: F, t20635: F, t20637: F, t20639: F, t20643: F, t20647: F, t20650: F, t20654: F, t20690: F) -> (F, F, F, F) {
    let t20886 = t3495 * t6534;
    let t20887 = t20886 * t1189;
    let t20889 = F::cast_from(0.11696447245269292414e1_f64) * t1196 * t20887;
    let t20890 = t12552 * t6518;
    let t20891 = t12555 * t1187;
    let t20892 = t20890 * t20891;
    let t20894 = F::cast_from(0.10254018858216406658e4_f64) * t1196 * t20892;
    let t20895 = t3520 * t6534;
    let t20896 = t20895 * t5206;
    let t20898 = F::cast_from(0.17315859105681463759e2_f64) * t1196 * t20896;
    let t20899 = t20690 + t20889 - t20894 - t20898 - t20571 + t20573 + t20576 - t20579 - t20582 + t20631 + t20633 + t20635 - t20637 + t20639 - t20643 + t20647 + t20650 + t20654;
    (t20889, t20894, t20898, t20899)
}

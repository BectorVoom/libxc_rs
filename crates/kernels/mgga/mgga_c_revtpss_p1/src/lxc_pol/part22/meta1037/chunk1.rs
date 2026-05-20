//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3627/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3627<F: Float>(t20394: F, t3531: F, t20896: F, t12571: F, t6556: F, t1196: F, t20890: F, t43977: F, t68631: F, t68633: F, t68636: F, t68640: F, t68683: F, t68686: F, t68689: F, t68692: F, t68694: F) -> (F, F, F, F, F) {
    let t68696 = F::cast_from(0.46785788981077169656e1_f64) * t3531 * t20394;
    let t68698 = F::cast_from(0.34631718211362927518e2_f64) * t3531 * t20896;
    let t68700 = F::cast_from(0.17315859105681463759e2_f64) * t12571 * t6556;
    let t68703 = F::cast_from(0.10254018858216406658e4_f64) * t1196 * t20890 * t43977;
    let t68704 = t68631 + t68633 + t68636 + t68640 - t68683 - t68686 - t68689 - t68692 - t68694 + t68696 - t68698 - t68700 - t68703;
    (t68696, t68698, t68700, t68703, t68704)
}

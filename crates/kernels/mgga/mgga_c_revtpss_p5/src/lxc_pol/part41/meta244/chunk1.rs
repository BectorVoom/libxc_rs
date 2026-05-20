//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 931/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk931<F: Float>(t1235: F, t1247: F, t1791: F, t1797: F, t3600: F, t3610: F, t3625: F, t3671: F, t3711: F, t484: F, t5254: F, t5256: F, t5266: F, t5274: F, t5293: F, t5323: F, t5327: F, t6595: F, t6598: F, t6602: F, t6611: F, t6619: F, t6625: F, t6631: F, t6635: F, t6640: F, t6647: F) -> F {
    let t6651 = F::cast_from(0.72409452821628889107e-2_f64) * t6595 * t484 - F::cast_from(0.22866142996303859718e-2_f64) * t6598 * t484 + F::cast_from(0.21437009059034868486e-3_f64) * t6602 * t484 - F::cast_from(0.22866142996303859718e-2_f64) * t5293 * t1797 - F::cast_from(0.15244095330869239812e-2_f64) * t5254 + F::cast_from(0.28582678745379824648e-3_f64) * t5256 + F::cast_from(0.42874018118069736972e-3_f64) * t3671 * t6611 + F::cast_from(0.22866142996303859718e-2_f64) * t5323 * t1791 + F::cast_from(0.42874018118069736972e-3_f64) * t5274 * t1797 + F::cast_from(0.28582678745379824648e-3_f64) * t3711 * t6619 + F::cast_from(0.21437009059034868486e-3_f64) * t1247 * t6625 + F::cast_from(0.42874018118069736972e-3_f64) * t3600 * t6631 - F::cast_from(0.21437009059034868486e-3_f64) * t3610 * t6635 - F::cast_from(0.28582678745379824648e-3_f64) * t3625 * t6640 - F::cast_from(0.42874018118069736972e-3_f64) * t5327 * t1791 - F::cast_from(0.21437009059034868486e-3_f64) * t1235 * t6647 + F::cast_from(0.28582678745379824648e-3_f64) * t5266;
    t6651
}

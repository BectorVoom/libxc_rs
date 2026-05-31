//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1079/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1079<F: Float>(t24543: F, t482: F, t13063: F, t1042: F, t22700: F, t344: F, t1261: F, t13062: F, t17377: F, t17529: F, t17569: F, t17572: F, t1808: F, t20784: F, t20787: F, t20789: F, t21143: F, t21272: F, t24535: F, t464: F, t5274: F, t5391: F, t6619: F, t6625: F, t6631: F, t6635: F, t6673: F) -> (F, F, F) {
    let t24544 = t482 * t24543;
    let t24545 = t24544 * t13063;
    let t24546 = t1042 * t24545;
    let t24551 = t22700 * t344;
    let t24562 = -F::cast_from(0.14481890564325777821e-1_f64) * t21272 * t1808 - F::cast_from(0.3811023832717309953e-2_f64) * t5391 * t6673 - F::cast_from(0.63517063878621832552e-3_f64) * t1261 * t24535 - F::cast_from(0.42874018118069736972e-3_f64) * t21143 * t1808 + F::cast_from(0.57165357490759649295e-3_f64) * t20784 - F::cast_from(0.42874018118069736972e-3_f64) * t20787 - F::cast_from(0.45732285992607719436e-2_f64) * t20789 + F::cast_from(0.21437009059034868486e-3_f64) * t13062 * t24546 + F::cast_from(0.85748036236139473944e-3_f64) * t17569 * t6619 - F::cast_from(77.0_f64) / F::cast_from(162.0_f64) * t24551 * t464 + F::cast_from(0.34299214494455789577e-2_f64) * t17529 * t6635 + F::cast_from(0.64311027177104605458e-3_f64) * t5274 * t6625 + F::cast_from(0.12862205435420921092e-2_f64) * t17572 * t6631 - F::cast_from(0.64311027177104605458e-3_f64) * t17377 * t6635;
    (t24544, t24546, t24562)
}

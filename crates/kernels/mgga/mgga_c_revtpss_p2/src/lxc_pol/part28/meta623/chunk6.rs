//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2213/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2213<F: Float>(t16035: F, t25580: F, t25569: F, t4817: F, t100019: F, t15592: F, t15622: F, t15847: F, t25517: F, t3317: F, t4783: F, t4831: F, t4902: F, t4912: F, t7132: F, t93543: F, t93597: F, t93602: F, t93611: F, t93616: F, t93667: F) -> F {
    let t100092 = F::cast_from(0.57165357490759649296e-3_f64) * t25580 * t16035;
    let t100097 = F::cast_from(0.3811023832717309953e-3_f64) * t25569 * t4817;
    let t100109 = -F::cast_from(0.85748036236139473944e-3_f64) * t93543 * t4912 - F::cast_from(0.3811023832717309953e-3_f64) * t93602 - t100092 + F::cast_from(0.45732285992607719436e-2_f64) * t3317 * t100019 * t4902 + t100097 + F::cast_from(0.57165357490759649296e-3_f64) * t25569 * t4831 + F::cast_from(0.28582678745379824648e-3_f64) * t7132 * t15847 + F::cast_from(0.17149607247227894789e-2_f64) * t93667 * t15622 + F::cast_from(0.28582678745379824648e-3_f64) * t25517 * t15592 - F::cast_from(0.30488190661738479624e-2_f64) * t93597 * t4783 + t93611 + F::cast_from(0.96545937095505185476e-2_f64) * t93616;
    t100109
}

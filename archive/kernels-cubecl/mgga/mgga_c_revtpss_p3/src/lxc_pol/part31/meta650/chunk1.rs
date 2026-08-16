//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2145/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2145<F: Float>(t19658: F, t7122: F, t19920: F, t25522: F, t27489: F, t4817: F, t100002: F, t100006: F, t100025: F, t100114: F, t1675: F, t19677: F, t19895: F, t20083: F, t25569: F, t27536: F, t4912: F, t6263: F, t6331: F, t93646: F) -> (F, F) {
    let t106877 = t7122 * t19658;
    let t106896 = t25522 * t19920;
    let t106906 = t27489 * t4817;
    let t106913 = F::cast_from(0.30488190661738479625e-2_f64) * t93646 * t6263 - F::cast_from(0.38110238327173099531e-3_f64) * t106896 - F::cast_from(0.28582678745379824648e-3_f64) * t25522 * t19677 + F::cast_from(0.57165357490759649296e-3_f64) * t25522 * t19895 - F::cast_from(0.57165357490759649296e-3_f64) * t25569 * t6331 - F::cast_from(0.30488190661738479625e-2_f64) * t100114 * t1675 + F::cast_from(0.38110238327173099531e-3_f64) * t106906 + F::cast_from(0.85748036236139473944e-3_f64) * t27536 * t20083 - F::cast_from(0.19055119163586549765e-3_f64) * t100002 + t100006 - F::cast_from(0.85748036236139473944e-3_f64) * t100025 * t4912;
    (t106877, t106913)
}

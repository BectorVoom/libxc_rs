//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2230/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2230<F: Float>(t104707: F, t1285: F, t12987: F, t7623: F, t5261: F, t1230: F, t29082: F, t29037: F, t3636: F, t104647: F, t1266: F, t17265: F, t17347: F, t17369: F, t17732: F, t29040: F, t3631: F, t3640: F, t3644: F, t7624: F, t97169: F) -> F {
    let t104721 = t1285 * t104707;
    let t104727 = t12987 * t7623;
    let t104732 = t5261 * t7623;
    let t104739 = t1230 * t29082;
    let t104742 = t29037 * t3636;
    let t104746 = F::cast_from(0.30488190661738479624e-2_f64) * t104721 * t3631 - F::cast_from(0.28582678745379824648e-3_f64) * t97169 - F::cast_from(0.28582678745379824648e-3_f64) * t7624 * t17369 - F::cast_from(0.25724410870841842183e-2_f64) * t104727 * t17347 + F::cast_from(0.85748036236139473944e-3_f64) * t29040 * t17265 - F::cast_from(0.57165357490759649296e-3_f64) * t104732 * t1266 - F::cast_from(0.28582678745379824648e-3_f64) * t29037 * t3640 - F::cast_from(0.57165357490759649296e-3_f64) * t29037 * t3644 + F::cast_from(0.30488190661738479624e-2_f64) * t104739 * t1266 - F::cast_from(0.3811023832717309953e-3_f64) * t104742 + F::cast_from(0.11433071498151929859e-2_f64) * t104647 * t17732;
    t104746
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2214/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2214<F: Float>(t1659: F, t25576: F, t27489: F, t3111: F, t11940: F, t7131: F, t16158: F, t7132: F, t1068: F, t15719: F, t1675: F, t25577: F, t3101: F, t3204: F, t4831: F, t4839: F, t93618: F, t93620: F, t93622: F, t93627: F, t93675: F) -> F {
    let t100114 = t1659 * t25576;
    let t100117 = t27489 * t3111;
    let t100121 = t11940 * t7131;
    let t100132 = F::cast_from(0.3811023832717309953e-3_f64) * t7132 * t16158;
    let t100133 = F::cast_from(0.10162730220579493208e-2_f64) * t93618 - F::cast_from(0.30488190661738479624e-2_f64) * t93620 - F::cast_from(0.19055119163586549765e-3_f64) * t93622 + F::cast_from(0.28582678745379824648e-3_f64) * t93627 - F::cast_from(0.30488190661738479624e-2_f64) * t100114 * t1068 + F::cast_from(0.3811023832717309953e-3_f64) * t100117 - F::cast_from(0.57165357490759649296e-3_f64) * t27489 * t3101 - F::cast_from(0.25724410870841842183e-2_f64) * t100121 * t15719 - F::cast_from(0.91464571985215438873e-2_f64) * t3204 * t25576 * t4839 - F::cast_from(0.30488190661738479624e-2_f64) * t93675 * t1675 - F::cast_from(0.30488190661738479624e-2_f64) * t25577 * t4831 + t100132;
    t100133
}

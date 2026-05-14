//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 937/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk937<F: Float>(t12487: F, t12552: F, t12555: F, t1196: F, t1188: F, t3520: F, t1294: F, t3568: F, t1277: F, t1204: F, t1269: F, t3584: F, t12295: F, t12292: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F) -> (F, F, F, F, F, F) {
    let t12592 = t12552 * t12487 * t12555;
    let t12594 = 0.10254018858216406658e4 * t1196 * t12592;
    let t12596 = t3520 * t12487 * t1188;
    let t12598 = 0.35089341735807877242e1 * t1196 * t12596;
    let t12599 = t3568 * t1294;
    let t12600 = t1277 * t12599;
    let t12603 = t1204 * t1269;
    let t12606 = t3584 * t1294;
    let t12607 = t1277 * t12606;
    let t12610 = 0.46096296296296296297e-1 * t12295;
    let t12621 = -t12610 + 0.19755555555555555556e-1 * t12297 + 0.9877777777777777778e-2 * t12299 - 0.29633333333333333334e-1 * t12301 - 0.14816666666666666667e-1 * t12303 + 0.16462962962962962963e-1 * t12307 - 0.59266666666666666668e-1 * t12310 - 0.29633333333333333334e-1 * t12292 + 0.88900000000000000002e-1 * t12314 + 0.88900000000000000002e-1 * t12317 + 0.14816666666666666667e-1 * t12320;
    (t12594, t12598, t12600, t12603, t12607, t12621)
}

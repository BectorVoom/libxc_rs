//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1036/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1036<F: Float>(t12599: F, t1277: F, t1204: F, t1269: F, t1294: F, t3584: F, t12295: F, t12292: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F) -> (F, F, F, F) {
    let t12600 = t1277 * t12599;
    let t12603 = t1204 * t1269;
    let t12606 = t3584 * t1294;
    let t12607 = t1277 * t12606;
    let t12610 = F::new(0.46096296296296296297e-1) * t12295;
    let t12621 = -t12610 + F::new(0.19755555555555555556e-1) * t12297 + F::new(0.9877777777777777778e-2) * t12299 - F::new(0.29633333333333333334e-1) * t12301 - F::new(0.14816666666666666667e-1) * t12303 + F::new(0.16462962962962962963e-1) * t12307 - F::new(0.59266666666666666668e-1) * t12310 - F::new(0.29633333333333333334e-1) * t12292 + F::new(0.88900000000000000002e-1) * t12314 + F::new(0.88900000000000000002e-1) * t12317 + F::new(0.14816666666666666667e-1) * t12320;
    (t12600, t12603, t12607, t12621)
}

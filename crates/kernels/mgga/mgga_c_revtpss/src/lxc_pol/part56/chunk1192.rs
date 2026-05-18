//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1192/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1192<F: Float>(t1243: F, t124694: F, t124772: F, t124780: F, t124931: F, t124984: F, t124996: F, t125017: F, t1276: F, t1287: F, t1294: F, t131699: F, t131703: F, t131962: F, t247: F, t29166: F, t29233: F, t29247: F, t29301: F, t33456: F, t33462: F, t33469: F, t33478: F, t33480: F, t34939: F, t3719: F, t5245: F, t5284: F, t5458: F, t5497: F, t8926: F, t8931: F) -> F {
    let t132005 = F::new(0.17347256376410398924e1) * t124996 * t131962 * t5458 - F::new(0.17347256376410398924e1) * t124780 * t29247 + F::new(0.17347256376410398924e1) * t124772 * t131962 * t29166 + F::new(0.51407763898592117355e1) * t33469 * t33478 * t34939 * t1294 + F::new(0.34694512752820797848e1) * t124931 * t29233 - F::new(0.17135921299530705785e1) * t33469 * t33462 * t8931 * t5245 - F::new(0.34271842599061411569e1) * t124984 * t131699 * t5458 - F::new(0.34271842599061411569e1) * t125017 * t131699 * t29166 - F::new(0.34694512752820797848e1) * t124694 * t29301 - F::new(0.8673628188205199462e0) * t33456 * t1243 * t5284 * t1287 - F::new(0.17135921299530705785e1) * t131703 * t33480 - F::new(0.28234466758480466999e-3) * t8926 * t247 * t3719 * t1276 * t5497;
    t132005
}

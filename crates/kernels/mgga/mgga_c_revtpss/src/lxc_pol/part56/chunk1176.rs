//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1176/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1176<F: Float>(t1243: F, t1802: F, t8944: F, t8938: F, t246: F, t3598: F, t33518: F, t5265: F, t33502: F, t5362: F, t104504: F, t1122: F, t124569: F, t124584: F, t124644: F, t124646: F, t124650: F, t1248: F, t1250: F, t13126: F, t1769: F, t2148: F, t33425: F, t33525: F, t34964: F, t3626: F, t5279: F, t5287: F, t5334: F, t5351: F, t5353: F) -> (F, F, F, F) {
    let t131394 = t1243 * t1802;
    let t131395 = t8944 * t131394;
    let t131396 = t8938 * t131395;
    let t131416 = t3598 * t246;
    let t131421 = t33518 * t5265;
    let t131423 = t33502 * t5362;
    let t131426 = F::cast_from(0.18822977838986977999e-3_f64) * t33425 * t3626 * t34964 * t1122 - F::cast_from(0.66110807482757352569e-3_f64) * t131396 * t33525 - F::cast_from(0.3718732920905101082e-3_f64) * t33518 * t5287 - F::cast_from(0.3718732920905101082e-3_f64) * t124584 * t5279 + F::cast_from(0.56468933516960933998e-3_f64) * t124650 * t124646 * t5351 * t104504 - F::cast_from(0.56468933516960933998e-3_f64) * t124644 * t124646 * t5353 + F::cast_from(0.56468933516960933998e-3_f64) * t124650 * t124646 * t1769 * t1248 * t1250 - F::cast_from(0.28234466758480466999e-3_f64) * t2148 * t13126 * t131416 * t124646 * t5334 - F::cast_from(0.24791552806034007213e-3_f64) * t131421 + F::cast_from(0.3718732920905101082e-3_f64) * t131423 - F::cast_from(0.37187329209051010821e-3_f64) * t124569;
    (t131394, t131395, t131416, t131426)
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 550/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk550<F: Float>(t169: F, t174: F, t4510: F, t4513: F, t740: F, t829: F, t1650: F, t2641: F, t167: F, t176: F, t833: F, t44: F, t2633: F, t234: F, t1071: F, t359: F, t1646: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t175 = t174 <= zeta_threshold;
    let t4517 = piecewise3(t170, 0.0, 4.0 / 9.0 * t4510 * t829 + 8.0 / 3.0 * t4513 * t740);
    let t4518 = t2641 * t1650;
    let t4521 = t176 * t167;
    let t4525 = piecewise3(t175, 0.0, 4.0 / 9.0 * t4518 * t833 - 8.0 / 3.0 * t4521 * t740);
    let t4527 = (t4517 + t4525) * t44;
    let t4532 = 2.0 * t2633;
    let t4533 = piecewise3(t170, 0.0, t4532);
    let t4534 = t234 * t4533;
    let t4546 = t359 * t1071;
    let t4547 = t1646 * t829;
    (t4518, t4521, t4527, t4533, t4534, t4546, t4547)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1206/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1206(t126148: f64, t126166: f64, t34068: f64, t689: f64, t119823: f64, t119841: f64, t121855: f64, t121869: f64, t126158: f64, t126164: f64, t2061: f64, t27265: f64, t27267: f64, t7048: f64, t7997: f64, t8645: f64, t8649: f64, t8650: f64) -> (f64, f64) {
    let t127662 = 0.3718732920905101082e-4_f64 * t126148;
    let t127675 = 0.13223814266738539448e-3_f64 * t126166;
    let t127676 = t34068 * t689;
    let t127677 = t119823 * t127676;
    let t127679 = t127662 - t119841 - t121855 + 0.57119737665102352616e0_f64 * t8649 * t8650 * t7997 * t7048 - 0.8673628188205199462e0_f64 * t8645 * t27267 + 0.225875734067843736e-2_f64 * t126158 + t121869 + 0.57119737665102352616e0_f64 * t8649 * t8650 * t2061 * t27265 - 0.74374658418102021639e-4_f64 * t126164 + t127675 - 0.76169170176413987216e-1_f64 * t127677;
    (t127676, t127679)
}

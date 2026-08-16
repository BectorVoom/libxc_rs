//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1113/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1113(t34610: f64, t34612: f64, t34617: f64, t34618: f64, t34621: f64, t34623: f64, t34627: f64, t34633: f64, t34636: f64, t34638: f64, t34640: f64, t37190: f64, t39402: f64, t39406: f64, t39412: f64, t39414: f64, t39418: f64, t39422: f64) -> f64 {
    let t39424 = -t34610 + 0.114609375e-1_f64 * t39402 + 0.12862205435420921092e-2_f64 * t39406 - t34612 - t34617 - 0.11321313224257494745e-1_f64 * t34618 + t34621 - t34623 - t34627 - t34633 - 0.94344276868812456204e-3_f64 * t34636 + 0.31448092289604152068e-3_f64 * t34638 + 0.28303283060643736861e-1_f64 * t34640 - 0.17149607247227894789e-2_f64 * t39412 - 0.17149607247227894789e-2_f64 * t39414 + t37190 + 0.94344276868812456204e-3_f64 * t39418 - 0.18868855373762491241e-2_f64 * t39422;
    t39424
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2626/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2626(t43859: f64, t44466: f64, t52313: f64, t52339: f64, t52343: f64, t64074: f64, t64076: f64, t64087: f64, t64089: f64, t71470: f64, t71472: f64, t71474: f64, t71477: f64, t71480: f64, t71483: f64, t71486: f64, t71489: f64, t71505: f64, t71508: f64, t71511: f64) -> f64 {
    let t73369 = -t52313 + 4.0_f64 / 81.0_f64 * t71470 - 2.0_f64 / 9.0_f64 * t71472 + 2.0_f64 / 3.0_f64 * t71474 - t71477 / 3.0_f64 + t71480 / 6.0_f64 + t71483 / 6.0_f64 - t71486 - t71489 - t44466 + 40.0_f64 / 81.0_f64 * t43859 + t71505 - 3.0_f64 * t71508 - 2.0_f64 / 9.0_f64 * t71511 - t52339 + t52343 - 2.0_f64 / 9.0_f64 * t64074 - 2.0_f64 / 3.0_f64 * t64076 + 4.0_f64 / 3.0_f64 * t64087 + 2.0_f64 * t64089;
    t73369
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1217/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1217(t225: f64, t24237: f64, t24235: f64, t2047: f64, t24305: f64, t24330: f64, t259: f64, t2713: f64, t2743: f64, t7107: f64, t82266: f64, t82282: f64, t82294: f64, t82296: f64, t866: f64, t9584: f64, t9590: f64, t9593: f64) -> f64 {
    let t85146 = t24237 * t225;
    let t85152 = t24235 * t225;
    let t85163 = 0.29608813203268075857e0_f64 * t82266 - 3.0_f64 * t24305 * t2743 - 6.0_f64 * t85146 * t866 - 6.0_f64 * t9593 * t7107 - 0.39478417604357434476e0_f64 * t82282 - 3.0_f64 * t85152 * t866 - 3.0_f64 * t9590 * t7107 + t9584 * t2047 * t259 - 0.31253747270116302294e0_f64 * t82294 - 0.69087230807625510332e0_f64 * t82296 + 6.0_f64 * t2713 * t24330;
    t85163
}

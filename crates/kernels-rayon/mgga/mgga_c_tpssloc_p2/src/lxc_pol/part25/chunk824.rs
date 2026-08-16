//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 824/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk824(t10588: f64, t901: f64, t276: f64, t285: f64, t2799: f64, t896: f64, t273: f64, t10311: f64, t10318: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10566: f64, t10569: f64, t10572: f64, t10575: f64, t10589: f64) -> (f64, f64, f64, f64) {
    let t10591 = t901 * t10588;
    let t10595 = 1.0_f64 / t276 / t285 / 4.0_f64;
    let t10596 = t2799 * t896;
    let t10597 = t10595 * t10596;
    let t10599 = 1.0_f64/pow_3_2(t273);
    let t10600 = t10599 * t10596;
    let t10602 = 0.16557e0_f64 * t10311 - 0.49671e0_f64 * t10318 - 0.40256666666666666668e0_f64 * t10556 + 0.20128333333333333333e0_f64 * t10558 - 0.60385000000000000001e0_f64 * t10560 + 0.30192500000000000001e0_f64 * t10562 - 0.33547222222222222222e0_f64 * t10566 + 0.12077e1_f64 * t10569 - 0.181155e1_f64 * t10572 - 0.301925e0_f64 * t10575 + 0.258925e1_f64 * t10589 + 0.16504875e0_f64 * t10591 + 0.19419375e1_f64 * t10597 - 0.412621875e-1_f64 * t10600;
    (t10591, t10597, t10600, t10602)
}

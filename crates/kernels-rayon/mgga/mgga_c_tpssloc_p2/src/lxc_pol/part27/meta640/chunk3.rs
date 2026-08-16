//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2166/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2166(t23110: f64, t25299: f64, t81651: f64, t23168: f64, t25313: f64, t13176: f64, t226: f64, t235: f64, t25256: f64, t25261: f64, t2617: f64, t4281: f64, t6658: f64, t81617: f64, t87150: f64, t87154: f64, t87155: f64, t87159: f64, t87166: f64, t87167: f64, t87171: f64, t87174: f64, t87177: f64, t87512: f64, t87517: f64, t9632: f64) -> f64 {
    let t87520 = t81651 * t23110 * t25299;
    let t87521 = 0.16449340668482264365e-1_f64 * t87520;
    let t87522 = t23168 * t25313;
    let t87523 = 0.76763589786250567036e-1_f64 * t87522;
    let t87524 = 0.16449340668482264365e-1_f64 * t87150 - t87154 + 0.26044789391763585244e-1_f64 * t87155 + 0.3289868133696452873e-1_f64 * t87159 + 2.0_f64 * t4281 * t25261 * t9632 + t87166 + t87167 - 0.19190897446562641759e-1_f64 * t81617 + 0.3289868133696452873e-1_f64 * t87171 - 0.16449340668482264365e-1_f64 * t87174 + 0.82246703342411321824e-2_f64 * t87177 - 2.0_f64 * t2617 * t25256 - 2.0_f64 * t13176 * t6658 + t226 * t235 * t87512 + 0.16449340668482264365e-1_f64 * t87517 - t87521 + t87523;
    t87524
}

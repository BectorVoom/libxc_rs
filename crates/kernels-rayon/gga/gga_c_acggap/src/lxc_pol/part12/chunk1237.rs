//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1237/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1237(t1410: f64, t157: f64, t2146: f64, t2152: f64, t2217: f64, t33175: f64, t33201: f64, t33208: f64, t33706: f64, t33778: f64, t36511: f64, t38280: f64, t38283: f64, t38285: f64, t38293: f64, t38309: f64, t406: f64, t7912: f64, t7931: f64, t8306: f64, t8307: f64, t8351: f64, t8400: f64, t8402: f64, t9003: f64, t9367: f64, t9440: f64) -> f64 {
    let t38311 = 0.17347256376410398924e1_f64 * t7912 * t9440 + t33201 - 0.17347256376410398924e1_f64 * t33778 * t8307 + 0.8673628188205199462e0_f64 * t8400 * t33175 * t8402 + t38280 - t38283 - 0.13170898365871023197e1_f64 * t38285 + 0.4336814094102599731e0_f64 * t9003 * t8351 + 0.4336814094102599731e0_f64 * t8400 * t8306 * t33706 - t38293 - 0.13170898365871023197e1_f64 * t33208 - 0.17347256376410398924e1_f64 * t7931 * t8306 * t36511 + 0.8673628188205199462e0_f64 * t2146 * t2152 * t2217 * t1410 * t157 + 0.8673628188205199462e0_f64 * t2146 * t2152 * t9367 * t406 * t157 + 0.13170898365871023197e1_f64 * t38309;
    t38311
}

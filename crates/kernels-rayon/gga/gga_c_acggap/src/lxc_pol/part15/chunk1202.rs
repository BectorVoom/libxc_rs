//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1202/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1202(t2176: f64, t5517: f64, t1614: f64, t9380: f64, t8397: f64, t9159: f64, t157: f64, t1838: f64, t2146: f64, t2152: f64, t2217: f64, t2245: f64, t33208: f64, t38280: f64, t38283: f64, t38285: f64, t38293: f64, t38309: f64, t38315: f64, t38319: f64, t38321: f64, t38771: f64, t8400: f64, t9427: f64, t9517: f64) -> f64 {
    let t41250 = t2176 * t5517;
    let t41258 = t9380 * t1614;
    let t41265 = t8397 * t9159;
    let t41267 = t38280 - t38283 - 0.26341796731742046394e1_f64 * t38285 - t38293 - 0.65854491829355115987e0_f64 * t33208 + 0.26341796731742046394e1_f64 * t38309 - 0.13170898365871023197e1_f64 * t41250 - 0.17347256376410398924e1_f64 * t8400 * t9427 * t38771 + t38315 - 0.4336814094102599731e0_f64 * t9517 * t2245 - t38319 - 0.13170898365871023197e1_f64 * t38321 + 0.13170898365871023197e1_f64 * t41258 + 0.4336814094102599731e0_f64 * t2146 * t2152 * t2217 * t1838 * t157 + 0.34694512752820797848e1_f64 * t41265;
    t41267
}

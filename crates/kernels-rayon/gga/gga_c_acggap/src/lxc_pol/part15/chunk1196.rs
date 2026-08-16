//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1196/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1196(t1814: f64, t2217: f64, t2385: f64, t939: f64, t1410: f64, t157: f64, t1938: f64, t2146: f64, t2152: f64, t33031: f64, t33037: f64, t33047: f64, t33053: f64, t38073: f64, t38077: f64, t38085: f64, t38089: f64, t38104: f64, t38111: f64, t8316: f64, t8400: f64, t8791: f64, t9003: f64, t9440: f64) -> (f64, f64) {
    let t41089 = t2217 * t1814;
    let t41106 = t939 * t2385;
    let t41111 = t33031 + 0.17347256376410398924e1_f64 * t38073 - 0.13170898365871023197e1_f64 * t38077 + 0.4336814094102599731e0_f64 * t2146 * t2152 * t41089 * t157 + t33037 - t38085 + t38089 + 0.17347256376410398924e1_f64 * t9003 * t9440 - 0.65854491829355115987e0_f64 * t8316 * t1938 - 0.52041769129231196772e1_f64 * t38104 + 0.13170898365871023197e1_f64 * t33047 + 0.8673628188205199462e0_f64 * t2146 * t2152 * t2385 * t1410 * t157 + 0.17347256376410398924e1_f64 * t33053 - 0.17347256376410398924e1_f64 * t8400 * t41106 * t8791 - 0.17347256376410398924e1_f64 * t38111;
    (t41089, t41111)
}

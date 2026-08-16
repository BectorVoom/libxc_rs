//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 849/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk849(t157: f64, t1838: f64, t2152: f64, t633: f64, t1937: f64, t2147: f64, t9980: f64, t8306: f64, t9508: f64, t1915: f64, t1938: f64, t2146: f64, t2222: f64, t2395: f64, t557: f64, t639: f64, t7931: f64, t8330: f64, t8339: f64, t8349: f64, t9003: f64, t9381: f64, t9391: f64, t9397: f64, t9399: f64, t9407: f64, t9409: f64, t9433: f64, t9437: f64, t9517: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10011 = t2152 * t633 * t1838 * t157;
    let t10017 = t633 * t1937;
    let t10018 = t2147 * t10017;
    let t10022 = t2152 * t9980 * t157;
    let t10025 = t8306 * t9508;
    let t10038 = 0.17347256376410398924e1_f64 * t9003 * t2395 - 0.65854491829355115987e0_f64 * t2222 * t1938 + 0.4336814094102599731e0_f64 * t2146 * t10011 + 0.13170898365871023197e1_f64 * t2222 * t1915 - 0.13170898365871023197e1_f64 * t9381 + 0.8673628188205199462e0_f64 * t2146 * t10018 + 0.4336814094102599731e0_f64 * t2146 * t10022 + t8330 - 0.17347256376410398924e1_f64 * t7931 * t10025 - t8339 + 0.13170898365871023197e1_f64 * t9397 - 0.13170898365871023197e1_f64 * t9399 - 0.17347256376410398924e1_f64 * t9407 + 0.17347256376410398924e1_f64 * t9409 - 0.4336814094102599731e0_f64 * t9517 * t639 - 0.13170898365871023197e1_f64 * t9391 * t557 + t8349 - 0.17347256376410398924e1_f64 * t9433 + 0.17347256376410398924e1_f64 * t9437;
    (t10011, t10017, t10018, t10022, t10025, t10038)
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 840/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk840(t1937: f64, t609: f64, t2147: f64, t157: f64, t2331: f64, t524: f64, t2152: f64, t119: f64, t1915: f64, t1938: f64, t2127: f64, t2146: f64, t2342: f64, t557: f64, t616: f64, t621: f64, t7950: f64, t7962: f64, t7996: f64, t8000: f64, t9003: f64, t9010: f64, t9055: f64, t9063: f64, t9073: f64, t9077: f64, t9517: f64, t9769: f64, t9774: f64, t9779: f64, t9790: f64) -> (f64, f64, f64, f64) {
    let t9793 = t609 * t1937;
    let t9794 = t2147 * t9793;
    let t9800 = t2331 * t524 * t157;
    let t9801 = t2152 * t9800;
    let t9804 = -0.13170898365871023197e1_f64 * t9010 * t557 + t7950 - 0.4336814094102599731e0_f64 * t9517 * t621 - 0.4336814094102599731e0_f64 * t616 * t9769 + 0.4336814094102599731e0_f64 * t2146 * t9774 + 0.34694512752820797848e1_f64 * t9055 + t7962 + 0.65854491829355115987e0_f64 * t119 * t9779 + 0.13170898365871023197e1_f64 * t2127 * t1915 - 0.65854491829355115987e0_f64 * t2127 * t1938 - 0.13170898365871023197e1_f64 * t9063 + 0.17347256376410398924e1_f64 * t9003 * t2342 + 0.17347256376410398924e1_f64 * t2146 * t9790 + 0.8673628188205199462e0_f64 * t2146 * t9794 + t7996 - t8000 - 0.13170898365871023197e1_f64 * t9073 - 0.34694512752820797848e1_f64 * t9077 + 0.8673628188205199462e0_f64 * t2146 * t9801;
    (t9793, t9794, t9801, t9804)
}

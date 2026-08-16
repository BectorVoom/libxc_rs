//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1273/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1273(t8397: f64, t9432: f64, t8998: f64, t9436: f64, t2226: f64, t40697: f64, t9154: f64, t119: f64, t150: f64, t187: f64, t2146: f64, t2147: f64, t2241: f64, t33320: f64, t33321: f64, t38487: f64, t38489: f64, t38493: f64, t40620: f64, t42181: f64, t463: f64, t7912: f64, t9003: f64, t9386: f64, t9971: f64, t9986: f64, t9991: f64) -> f64 {
    let t42261 = t8397 * t9432;
    let t42263 = t8998 * t9436;
    let t42269 = t40697 * t2226;
    let t42280 = t8998 * t9154;
    let t42284 = 0.17347256376410398924e1_f64 * t7912 * t9986 - 0.17347256376410398924e1_f64 * t42261 - t38487 + 0.17347256376410398924e1_f64 * t42263 + t38489 + 0.8673628188205199462e0_f64 * t2146 * t2147 * t9971 * t463 - 0.8673628188205199462e0_f64 * t42269 + 0.8673628188205199462e0_f64 * t7912 * t9991 - t38493 + 0.8673628188205199462e0_f64 * t9003 * t9386 - t33320 + 0.65854491829355115987e0_f64 * t119 * t42181 * t150 * t187 + 0.8673628188205199462e0_f64 * t33321 - 0.34694512752820797848e1_f64 * t42280 + 0.4336814094102599731e0_f64 * t40620 * t2241;
    t42284
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 971/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk971(t1401: f64, t32284: f64, t1405: f64, t8591: f64, t1412: f64, t241: f64, t125: f64, t1353: f64, t246: f64, t32246: f64, t32247: f64, t32252: f64, t32258: f64, t32262: f64, t32267: f64, t32271: f64, t32273: f64, t32280: f64, t8586: f64, t8706: f64) -> (f64, f64, f64, f64, f64) {
    let t32285 = t32284 * t1401;
    let t32287 = t8591 * t1405;
    let t32288 = 0.86770434821119025247e-3_f64 * t32287;
    let t32289 = t241 * t1412;
    let t32291 = t246 * t125 * t1353;
    let t32292 = t32289 * t32291;
    let t32293 = t8591 * t32292;
    let t32295 = t32246 + 0.57119737665102352616e0_f64 * t32247 * t8586 - 0.17135921299530705785e1_f64 * t8706 * t32252 - 0.11423947533020470523e1_f64 * t8706 * t32258 + 0.11423947533020470523e1_f64 * t8706 * t32262 + t32267 - t32271 - 0.1859366460452550541e-3_f64 * t32273 + 0.3718732920905101082e-3_f64 * t32280 + 0.3718732920905101082e-3_f64 * t32285 + t32288 + 0.7437465841810202164e-3_f64 * t32293;
    (t32288, t32289, t32291, t32292, t32295)
}

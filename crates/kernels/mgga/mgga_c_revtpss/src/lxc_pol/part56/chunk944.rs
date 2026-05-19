//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 944/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk944<F: Float>(t1401: F, t32284: F, t1405: F, t8591: F, t1412: F, t241: F, t125: F, t1353: F, t246: F, t32246: F, t32247: F, t32252: F, t32258: F, t32262: F, t32267: F, t32271: F, t32273: F, t32280: F, t8586: F, t8706: F) -> (F, F, F, F, F) {
    let t32285 = t32284 * t1401;
    let t32287 = t8591 * t1405;
    let t32288 = F::cast_from(0.86770434821119025247e-3_f64) * t32287;
    let t32289 = t241 * t1412;
    let t32291 = t246 * t125 * t1353;
    let t32292 = t32289 * t32291;
    let t32293 = t8591 * t32292;
    let t32295 = t32246 + F::cast_from(0.57119737665102352616e0_f64) * t32247 * t8586 - F::cast_from(0.17135921299530705785e1_f64) * t8706 * t32252 - F::cast_from(0.11423947533020470523e1_f64) * t8706 * t32258 + F::cast_from(0.11423947533020470523e1_f64) * t8706 * t32262 + t32267 - t32271 - F::cast_from(0.1859366460452550541e-3_f64) * t32273 + F::cast_from(0.3718732920905101082e-3_f64) * t32280 + F::cast_from(0.3718732920905101082e-3_f64) * t32285 + t32288 + F::cast_from(0.7437465841810202164e-3_f64) * t32293;
    (t32288, t32289, t32291, t32292, t32295)
}

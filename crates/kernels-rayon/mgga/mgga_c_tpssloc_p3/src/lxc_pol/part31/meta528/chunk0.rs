//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1742/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1742(t1824: f64, t5318: f64, t1372: f64, t6387: f64, t6414: f64, t19731: f64, t562: f64, t20063: f64, t3701: f64, t1484: f64, t2752: f64, t17083: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t57545 = t5318 * t1824;
    let t57607 = t1372 * t6387;
    let t57618 = t1372 * t6414;
    let t57704 = t562 * t19731;
    let t57806 = t20063 * t3701;
    let t57911 = t2752 * t1484;
    let t58143 = t17083 * t225;
    (t57545, t57607, t57618, t57704, t57806, t57911, t58143)
}

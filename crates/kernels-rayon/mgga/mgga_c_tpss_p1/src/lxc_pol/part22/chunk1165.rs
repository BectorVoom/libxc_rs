//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1165/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1165(t1629: f64, t3234: f64, t762: f64, t10161: f64, t10166: f64, t1213: f64, t1244: f64, t12996: f64, t13000: f64, t13004: f64, t13006: f64, t13009: f64, t13013: f64, t13015: f64, t13018: f64, t13021: f64, t13023: f64, t3244: f64, t4413: f64) -> f64 {
    let t13027 = t762 * t1629 * t3234;
    let t13030 = -t1213 * t12996 / 48.0_f64 + t4413 * t13000 / 1536.0_f64 - t13004 + t13006 - 35.0_f64 / 108.0_f64 * t10161 - t10166 - t4413 * t13009 / 384.0_f64 + t13013 - t1244 * t13015 / 768.0_f64 - 119.0_f64 / 3456.0_f64 * t13018 - t13021 + t3244 * t13023 / 8.0_f64 + t3244 * t13027 / 16.0_f64;
    t13030
}

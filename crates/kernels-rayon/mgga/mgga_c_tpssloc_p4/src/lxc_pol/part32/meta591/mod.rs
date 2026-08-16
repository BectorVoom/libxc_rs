//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta591 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1979;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta591(t1824: f64, t5318: f64, t1372: f64, t6387: f64, t6414: f64, t19731: f64, t562: f64, t20063: f64, t3701: f64, t1484: f64, t2752: f64, t17083: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t57545, t57607, t57618, t57704, t57806, t57911, t58143) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1979(t1824, t5318, t1372, t6387, t6414, t19731, t562, t20063, t3701, t1484, t2752, t17083, t225);
    (t57545, t57607, t57618, t57704, t57806, t57911, t58143)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta310 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1333;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta310(t2791: f64, t888: f64, t2929: f64, t938: f64, t10523: f64, t315: f64, t10544: f64, t1043: f64, t676: f64, t248: f64, t884: f64, t1041: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t10817, t10825, t10828, t10832, t10868, t10870, t10871) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1333(t2791, t888, t2929, t938, t10523, t315, t10544, t1043, t676, t248, t884, t1041);
    (t10817, t10825, t10828, t10832, t10868, t10870, t10871)
}

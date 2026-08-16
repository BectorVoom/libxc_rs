//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta408 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1547;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta408(t10165: f64, t18070: f64, t225: f64, t5915: f64, t1049: f64, t5872: f64, t3201: f64, t3188: f64, t1057: f64, t18028: f64, t1615: f64, t4657: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t18071, t18074, t18080, t18081, t18083, t18086, t18088) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1547(t10165, t18070, t225, t5915, t1049, t5872, t3201, t3188, t1057, t18028, t1615, t4657);
    (t18071, t18074, t18080, t18081, t18083, t18086, t18088)
}

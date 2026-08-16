//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta284 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1282;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1283;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta284(t590: f64, t60: f64, t192: f64, t533: f64, t1390: f64, t2018: f64, t16: f64, t2: f64, t591: f64, t9: f64, t21: f64, t587: f64, t14: f64, t598: f64, t2230: f64, t594: f64, t2229: f64, t3: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t8705, t8944, t8945, t9212, t9214, t9216) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1282(t590, t60, t192, t533, t1390, t2018, t16, t2, t591, t9, t21, t587);
        let (t9218, t9220, t9222, t9223) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1283(t14, t598, t2230, t594, t2229, t3);
    (t8705, t8944, t8945, t9212, t9214, t9216, t9218, t9220, t9222, t9223)
}

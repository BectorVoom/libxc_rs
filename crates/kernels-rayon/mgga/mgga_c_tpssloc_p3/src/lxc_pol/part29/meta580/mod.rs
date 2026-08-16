//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1998;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1999;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta580(t22705: f64, t22852: f64, t550: f64, t80786: f64, t22823: f64, t281: f64, t22855: f64, t3862: f64, t6940: f64, t1358: f64, t22836: f64, t22690: f64, t3787: f64, t3792: f64, t236: f64, t3850: f64, t1361: f64, t22792: f64, t3719: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t80789, t80791, t80792, t80794, t80796, t80798) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1998(t22705, t22852, t550, t80786, t22823, t281, t22855, t3862, t6940, t1358, t22836, t22690, t3787);
        let (t80801, t80807, t80814) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1999(t22852, t3792, t80786, t80798, t22705, t236, t3850, t550, t1361, t22690, t22792, t3719);
    (t80789, t80791, t80792, t80794, t80796, t80798, t80801, t80807, t80814)
}

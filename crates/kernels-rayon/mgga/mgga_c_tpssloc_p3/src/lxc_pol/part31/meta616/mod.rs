//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1863;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta616(t5464: f64, t666: f64, t81446: f64, t1453: f64, t4067: f64, t22473: f64, t22470: f64, t5488: f64, t19529: f64, t6530: f64, t7684: f64, t8944: f64, t1390: f64, t19631: f64, t1845: f64, t5356: f64, t22674: f64, t28191: f64, t80681: f64, t1985: f64, t22666: f64, t28232: f64, t26331: f64, t26333: f64, t90566: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96716, t96719, t96721, t96724, t96726, t96797) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1863(t5464, t666, t81446, t1453, t4067, t22473, t22470, t5488, t19529, t6530, t7684, t8944);
        let (t96824, t96830, t96848, t96851, t96854) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1864(t1390, t19631, t1845, t5356, t22674, t28191, t80681, t1985, t22666, t28232, t26331, t26333, t90566);
    (t96716, t96719, t96721, t96724, t96726, t96797, t96824, t96830, t96848, t96851, t96854)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1334;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1335;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta357(t4290: f64, t808: f64, t13380: f64, t4182: f64, t68: f64, t9971: f64, t226: f64, t13263: f64, t4282: f64, t2633: f64, t9632: f64, t2732: f64, t4234: f64, t2679: f64, t4295: f64, t1519: f64, t2627: f64, t10076: f64, t1510: f64, t13381: f64, t13385: f64, t13388: f64, t2617: f64, t2729: f64, t2733: f64, t2736: f64, t4166: f64, t4281: f64, t4291: f64, t4292: f64, t4296: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t13390, t13393, t13397, t13398, t13401, t13404, t13407) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1334(t4290, t808, t13380, t4182, t68, t9971, t226, t13263, t4282, t2633, t9632, t2732, t4234);
        let t13425 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1335(t2679, t4295, t1519, t2627, t2633, t10076, t1510, t13381, t13385, t13388, t13390, t13393, t13397, t13398, t13401, t13404, t13407, t2617, t2729, t2733, t2736, t4166, t4281, t4291, t4292, t4296, t812);
    (t13390, t13393, t13397, t13398, t13401, t13404, t13425)
}

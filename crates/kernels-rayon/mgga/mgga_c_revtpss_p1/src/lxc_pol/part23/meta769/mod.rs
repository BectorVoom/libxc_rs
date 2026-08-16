//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta769 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2569;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2570;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta769(t1284: f64, t17288: f64, t3624: f64, t1260: f64, t17289: f64, t13032: f64, t17524: f64, t12881: f64, t5381: f64, t17861: f64, t17416: f64, t3647: f64, t11262: f64, t1247: f64, t5286: f64, t13099: f64, t43776: f64, t12909: f64, t17395: f64, t44546: f64, t5331: f64, t5334: f64, t17528: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57040, t57053, t57056, t57094, t57100, t57118) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2569(t1284, t17288, t3624, t1260, t17289, t13032, t17524, t12881, t5381, t17861, t17416, t3647);
        let (t57126, t57136, t57147, t57223, t57229) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2570(t11262, t1247, t5286, t13099, t43776, t12909, t17395, t44546, t5331, t5334, t13032, t17528);
    (t57040, t57053, t57056, t57094, t57100, t57118, t57126, t57136, t57147, t57223, t57229)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta624 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1948;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1949;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta624(t26233: f64, t3853: f64, t1827: f64, t80914: f64, t1811: f64, t80775: f64, t7709: f64, t80766: f64, t22797: f64, t5227: f64, t22804: f64, t26277: f64, t16308: f64, t22833: f64, t16123: f64, t2002: f64, t559: f64, t1307: f64, t1377: f64, t22633: f64, t22635: f64, t5353: f64, t26215: f64, t80650: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91391, t91394, t91398, t91400, t91402, t91404) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1948(t26233, t3853, t1827, t80914, t1811, t80775, t7709, t80766, t22797, t5227, t22804, t26277);
        let (t91413, t91416, t91449, t91455) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1949(t16308, t22833, t16123, t2002, t559, t1307, t1377, t22633, t22635, t5353, t26215, t80650);
    (t91391, t91394, t91398, t91400, t91402, t91404, t91413, t91416, t91449, t91455)
}

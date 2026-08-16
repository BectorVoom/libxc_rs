//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1832;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta589(t1827: f64, t80991: f64, t22765: f64, t5289: f64, t22764: f64, t5234: f64, t1354: f64, t26298: f64, t80958: f64, t22779: f64, t26319: f64, t1358: f64, t26248: f64, t3862: f64, t7715: f64, t22705: f64, t22852: f64, t236: f64, t5286: f64, t550: f64, t26245: f64, t80791: f64, t80867: f64, t26271: f64, t80836: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91281, t91283, t91285, t91286, t91290, t91300, t91303) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1832(t1827, t80991, t22765, t5289, t22764, t5234, t1354, t26298, t80958, t22779, t26319, t1358, t26248);
        let (t91305, t91310, t91312, t91314, t91323) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1833(t3862, t7715, t22705, t22852, t236, t5286, t550, t26245, t80791, t80867, t26271, t80836);
    (t91281, t91283, t91285, t91286, t91290, t91300, t91303, t91305, t91310, t91312, t91314, t91323)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta710 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2306;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2307;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta710(t57965: f64, t40722: f64, t40733: f64, t57992: f64, t185: f64, t67060: f64, t707: f64, t21066: f64, t2752: f64, t145: f64, t67083: f64, t20767: f64, t751: f64, t16596: f64, t16662: f64, t17116: f64, t1877: f64, t2522: f64, t39483: f64, t40732: f64, t4310: f64, t46237: f64, t868: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t67137, t67141, t67146, t67147, t67153, t67154, t67158, t67159) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2306(t57965, t40722, t40733, t57992, t185, t67060, t707, t21066, t2752, t145, t67083, t20767, t751);
        let t67160 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2307(t16596, t16662, t17116, t1877, t2522, t39483, t40732, t4310, t46237, t67146, t67147, t67153, t67154, t67158, t67159, t868);
    (t67137, t67141, t67146, t67147, t67153, t67158, t67159, t67160)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta631 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2414;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2415;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta631(t2649: f64, t41115: f64, t2553: f64, t2632: f64, t10024: f64, t809: f64, t2614: f64, t2693: f64, t238: f64, t244: f64, t248: f64, t40445: f64, t212: f64, t2586: f64, t9523: f64, t9525: f64, t9577: f64, t116: f64, t2379: f64, t207: f64, t40419: f64, t9538: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41116, t41123, t41130, t41134, t41139) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2414(t2649, t41115, t2553, t2632, t10024, t809, t2614, t2693, t238, t244, t248, t40445);
        let (t41142, t41144, t41146, t41149, t41155) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2415(t212, t2553, t2586, t9523, t9525, t9577, t116, t244, t2379, t207, t40419, t9538);
    (t41116, t41123, t41130, t41134, t41139, t41142, t41144, t41146, t41149, t41155)
}

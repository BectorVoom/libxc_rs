//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2087;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2088;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta546(t10041: f64, t2563: f64, t2678: f64, t776: f64, t222: f64, t39934: f64, t2617: f64, t9637: f64, t2649: f64, t2691: f64, t812: f64, t815: f64, t10024: f64, t809: f64, t10017: f64, t838: f64, t2614: f64, t2693: f64, t238: f64, t244: f64, t248: f64, t40445: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41088, t41090, t41096, t41107, t41108, t41115) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2087(t10041, t2563, t2678, t776, t222, t39934, t2617, t9637, t2649, t2691, t812, t815);
        let (t41116, t41130, t41132, t41134, t41139) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2088(t2649, t41115, t10024, t809, t10017, t838, t2614, t2693, t238, t244, t248, t40445);
    (t41088, t41090, t41096, t41107, t41108, t41115, t41116, t41130, t41132, t41134, t41139)
}

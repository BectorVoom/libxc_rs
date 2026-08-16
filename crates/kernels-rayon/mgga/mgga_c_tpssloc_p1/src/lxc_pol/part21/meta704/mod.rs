//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta704 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2535;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2536;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta704(t1036: f64, t13751: f64, t10422: f64, t14229: f64, t3070: f64, t14234: f64, t42488: f64, t1022: f64, t4649: f64, t41666: f64, t43398: f64, t14036: f64, t13969: f64, t13976: f64, t3130: f64, t1041: f64, t14183: f64, t10471: f64, t47840: f64, t10479: f64, t10908: f64, t4641: f64, t10216: f64, t13797: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48446, t48460, t48463, t48477, t48496, t48548) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2535(t1036, t13751, t10422, t14229, t3070, t14234, t42488, t1022, t4649, t41666, t43398, t14036);
        let (t48564, t48567, t48569, t48570, t48574, t48585) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2536(t13969, t13976, t3130, t1041, t14183, t10471, t47840, t10479, t10908, t4641, t10216, t13797);
    (t48446, t48460, t48463, t48477, t48496, t48548, t48564, t48567, t48569, t48570, t48574, t48585)
}

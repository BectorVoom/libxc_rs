//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta737 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2598;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2599;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta737(t11698: f64, t15569: f64, t15498: f64, t3523: f64, t15495: f64, t3572: f64, t1227: f64, t1653: f64, t248: f64, t45293: f64, t15591: f64, t15643: f64, t3490: f64, t1734: f64, t3507: f64, t11721: f64, t11786: f64, t5005: f64, t15730: f64, t3536: f64, t15594: f64, t1174: f64, t14726: f64, t44562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52664, t52666, t52674, t52680, t52682, t52684) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2598(t11698, t15569, t15498, t3523, t15495, t3572, t1227, t1653, t248, t45293, t15591, t15643, t3490);
        let (t52696, t52704, t52725, t52731, t52733, t52751) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2599(t1734, t3507, t11721, t11786, t5005, t15730, t3536, t15594, t3523, t1174, t14726, t44562);
    (t52664, t52666, t52674, t52680, t52682, t52684, t52696, t52704, t52725, t52731, t52733, t52751)
}

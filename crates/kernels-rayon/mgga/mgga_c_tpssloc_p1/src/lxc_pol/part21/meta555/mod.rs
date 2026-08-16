//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2250;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2251;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta555(t1100: f64, t18730: f64, t1107: f64, t11243: f64, t5992: f64, t1102: f64, t4756: f64, t4764: f64, t3287: f64, t5999: f64, t11265: f64, t4748: f64, t11211: f64, t11372: f64, t14702: f64, t14705: f64, t14711: f64, t3270: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18731, t18742, t18746, t18747, t18749, t18751, t18752, t18754, t18755, t18757) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2250(t1100, t18730, t1107, t11243, t5992, t1102, t4756, t4764, t3287, t5999, t11265, t4748);
        let (t18759, t18761) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2251(t11211, t11372, t14702, t14705, t14711, t18742, t18747, t18749, t18752, t18755, t18757, t3270, t5999);
    (t18731, t18742, t18746, t18747, t18749, t18751, t18752, t18754, t18755, t18757, t18759, t18761)
}

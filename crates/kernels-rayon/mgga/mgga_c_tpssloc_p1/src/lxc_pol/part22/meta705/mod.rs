//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta705 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2294;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2295;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta705(t18375: f64, t3536: f64, t11697: f64, t18968: f64, t3577: f64, t11539: f64, t1174: f64, t18232: f64, t18215: f64, t11665: f64, t18371: f64, t15569: f64, t15572: f64, t1244: f64, t3068: f64, t478: f64, t6163: f64, t18386: f64, t15608: f64, t15740: f64, t6183: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t66554, t66566, t66571, t66575, t66597, t66599) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2294(t18375, t3536, t11697, t18968, t3577, t11539, t1174, t18232, t18215, t11665, t18371, t15569, t15572);
        let (t66622, t66646, t66648, t66668) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2295(t1244, t3068, t478, t6163, t11697, t18386, t3577, t15608, t15740, t1174, t6183, t698);
    (t66554, t66566, t66571, t66575, t66597, t66599, t66622, t66646, t66648, t66668)
}

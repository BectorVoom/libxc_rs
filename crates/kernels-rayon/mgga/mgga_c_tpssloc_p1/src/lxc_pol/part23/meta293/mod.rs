//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta293 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1011;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta293(t21114: f64, t932: f64, t1557: f64, t17195: f64, t4354: f64, t5727: f64, t13520: f64, t5730: f64, t21252: f64, t2844: f64, t10661: f64, t10675: f64, t10676: f64, t21120: f64, t21124: f64, t21128: f64, t21132: f64, t21136: f64, t21140: f64, t21142: f64, t21144: f64, t21147: f64, t21150: f64, t21153: f64, t21156: f64, t13598: f64, t13642: f64, t17149: f64, t17165: f64, t17175: f64, t17286: f64, t17288: f64, t17290: f64, t21161: f64, t21168: f64, t21181: f64, t21183: f64, t21186: f64, t21188: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21259, t21263, t21265, t21267, t21268, t21270, t21283) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1011(t21114, t932, t1557, t17195, t4354, t5727, t13520, t5730, t21252, t2844, t10661, t10675, t10676, t21120, t21124, t21128, t21132, t21136, t21140, t21142, t21144, t21147, t21150, t21153, t21156);
        let t21298 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1012(t13598, t13642, t17149, t17165, t17175, t17286, t17288, t17290, t21161, t21168, t21181, t21183, t21186, t21188);
    (t21259, t21263, t21265, t21267, t21268, t21270, t21283, t21298)
}

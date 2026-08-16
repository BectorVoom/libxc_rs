//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta696 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2277;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2278;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta696(t3500: f64, t3503: f64, t65539: f64, t1210: f64, t15734: f64, t5005: f64, t19047: f64, t3572: f64, t11818: f64, t248: f64, t3506: f64, t6225: f64, t11539: f64, t1174: f64, t18211: f64, t3540: f64, t6170: f64, t19015: f64, t3577: f64, t45124: f64, t6158: f64, t15730: f64, t5002: f64, t1226: f64, t18573: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t65541, t65545, t65552, t65554, t65558) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2277(t3500, t3503, t65539, t1210, t15734, t5005, t19047, t3572, t11818, t248, t3506, t6225);
        let (t65567, t65581, t65598, t65600, t65605, t65607) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2278(t11539, t1174, t18211, t3540, t6170, t19015, t3577, t45124, t6158, t15730, t5002, t1226, t18573);
    (t65541, t65545, t65552, t65554, t65558, t65567, t65581, t65598, t65600, t65605, t65607)
}

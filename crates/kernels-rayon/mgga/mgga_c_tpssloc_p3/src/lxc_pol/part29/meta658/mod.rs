//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta658 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2185;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2186;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta658(t12823: f64, t7468: f64, t26003: f64, t4034: f64, t26351: f64, t6883: f64, t1992: f64, t26355: f64, t80650: f64, t22635: f64, t26354: f64, t3911: f64, t22751: f64, t26186: f64, t26190: f64, t26356: f64, t6914: f64, t1385: f64, t3886: f64, t5353: f64, t3888: f64, t55118: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90454, t90456, t90460, t90462, t90466) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2185(t12823, t7468, t26003, t4034, t26351, t6883, t1992, t26355, t80650, t22635, t26354, t3911);
        let (t90469, t90471, t90473, t90477, t90485) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2186(t22751, t26186, t26190, t26356, t6914, t1385, t1992, t22635, t3886, t5353, t3888, t55118);
    (t90454, t90456, t90460, t90462, t90466, t90469, t90471, t90473, t90477, t90485)
}

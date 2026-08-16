//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2014;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2015;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta590(t22734: f64, t81159: f64, t22899: f64, t6914: f64, t22715: f64, t6887: f64, t6970: f64, t22751: f64, t22883: f64, t12225: f64, t22641: f64, t22690: f64, t6969: f64, t1338: f64, t22870: f64, t22886: f64, t22892: f64, t22893: f64, t22887: f64, t268: f64, t547: f64, t6559: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81160, t81184, t81186, t81187, t81189, t81195, t81197) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2014(t22734, t81159, t22899, t6914, t22715, t6887, t6970, t22751, t22883, t12225, t22641, t22690, t6969);
        let (t81199, t81216, t81218, t81228) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2015(t1338, t22870, t22886, t22892, t22893, t22751, t22887, t268, t547, t6559);
    (t81160, t81184, t81186, t81187, t81189, t81195, t81197, t81199, t81216, t81218, t81228)
}

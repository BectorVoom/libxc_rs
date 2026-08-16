//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2282;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2283;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta629(t47160: f64, t41291: f64, t12932: f64, t2427: f64, t13133: f64, t2430: f64, t145: f64, t185: f64, t46191: f64, t45872: f64, t707: f64, t12886: f64, t706: f64, t708: f64, t41295: f64, t157: f64, t41284: f64, t12940: f64, t12923: f64, t12939: f64, t2244: f64, t12892: f64, t12908: f64, t2250: f64, t4194: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47161, t47162, t47164, t47166, t47168, t47171, t47172) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2282(t47160, t41291, t12932, t2427, t13133, t2430, t145, t185, t46191, t45872, t707, t12886, t706);
        let (t47174, t47175, t47178, t47181, t47183, t47185) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2283(t47172, t708, t41295, t157, t41284, t12940, t12923, t12939, t2244, t12892, t12908, t2250, t4194);
    (t47161, t47162, t47164, t47166, t47168, t47171, t47174, t47175, t47178, t47181, t47183, t47185)
}

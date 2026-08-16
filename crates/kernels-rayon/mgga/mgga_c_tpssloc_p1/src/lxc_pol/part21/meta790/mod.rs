//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta790 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2749;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta790(t41279: f64, t5499: f64, t12945: f64, t4205: f64, t46208: f64, t4194: f64, t5398: f64, t607: f64, t750: f64, t46217: f64, t13130: f64, t32: f64, t5519: f64, t2659: f64, t16606: f64, t2379: f64, t39463: f64, t39468: f64, t40714: f64, t40716: f64, t4314: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57959, t57961, t57962, t57966, t57970, t57972, t57973) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2749(t41279, t5499, t12945, t4205, t46208, t4194, t5398, t607, t750, t46217, t13130, t32, t5519);
        let (t57975, t57976) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2750(t2659, t57973, t16606, t2379, t39463, t39468, t40714, t40716, t4314, t57959, t57961, t57962, t57966, t57970, t57972);
    (t57959, t57961, t57962, t57966, t57970, t57972, t57975, t57976)
}

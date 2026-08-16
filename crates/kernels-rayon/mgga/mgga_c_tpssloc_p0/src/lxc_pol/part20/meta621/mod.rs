//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2236;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2237;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta621(t40817: f64, t157: f64, t41279: f64, t4196: f64, t4205: f64, t9868: f64, t13130: f64, t2427: f64, t41251: f64, t10121: f64, t13191: f64, t1877: f64, t2523: f64, t39563: f64, t39585: f64, t39590: f64, t39593: f64, t4307: f64, t4314: f64, t193: f64, t776: f64, t12908: f64, t13127: f64, t3966: f64, t4194: f64, t607: f64, t750: f64, t12606: f64, t184: f64, t4202: f64, t9912: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46331, t46334, t46336, t46338, t46339, t46340) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2236(t40817, t157, t41279, t4196, t4205, t9868, t13130, t2427, t41251, t10121, t13191, t1877, t2523, t39563, t39585, t39590, t39593, t4307, t4314);
        let (t46341, t46345, t46349, t46353, t46355) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2237(t193, t776, t12908, t13127, t3966, t4194, t607, t750, t12606, t184, t4202, t9912);
    (t46331, t46334, t46336, t46338, t46339, t46340, t46341, t46345, t46349, t46353, t46355)
}

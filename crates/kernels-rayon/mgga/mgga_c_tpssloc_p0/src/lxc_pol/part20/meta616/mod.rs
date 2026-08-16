//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2224;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2225;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2226;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta616(t46137: f64, t40667: f64, t40670: f64, t40673: f64, t40680: f64, t40682: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t40679: f64, t40685: f64, t40: f64, t10913: f64, t12939: f64, t4195: f64, t12606: f64, t12862: f64, t12865: f64, t1409: f64, t2244: f64, t2250: f64, t2433: f64, t3966: f64, t40632: f64, t4080: f64, t45872: f64, t607: f64, t73: f64, t9258: f64, t9288: f64, t9427: f64, zeta_threshold: f64, t52: f64, t12874: f64, t12877: f64, t2440: f64, t40647: f64, t4087: f64, t76: f64, t9438: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46138, t46140, t46141, t46142, t46143, t46144, t46145) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2224(t46137, t40667, t40670, t40673, t40680, t40682, t39309, t39312, t39316, t39320, t40679, t40685);
        let (t46152, t46171) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2225(t40, t10913, t12939, t4195, t12606, t12862, t12865, t1409, t2244, t2250, t2433, t3966, t40632, t4080, t45872, t607, t73, t9258, t9288, t9427, zeta_threshold);
        let t46190 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2226(t52, t10913, t12606, t12874, t12877, t1409, t2244, t2250, t2440, t3966, t40647, t4087, t45872, t607, t76, t9258, t9288, t9438, zeta_threshold);
    (t46138, t46140, t46141, t46142, t46143, t46144, t46145, t46152, t46171, t46190)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2224;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2225;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2226;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta616<F: Float>(t46137: F, t40667: F, t40670: F, t40673: F, t40680: F, t40682: F, t39309: F, t39312: F, t39316: F, t39320: F, t40679: F, t40685: F, t40: F, t10913: F, t12939: F, t4195: F, t12606: F, t12862: F, t12865: F, t1409: F, t2244: F, t2250: F, t2433: F, t3966: F, t40632: F, t4080: F, t45872: F, t607: F, t73: F, t9258: F, t9288: F, t9427: F, zeta_threshold: F, t52: F, t12874: F, t12877: F, t2440: F, t40647: F, t4087: F, t76: F, t9438: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46138, t46140, t46141, t46142, t46143, t46144, t46145) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2224::<F>(t46137, t40667, t40670, t40673, t40680, t40682, t39309, t39312, t39316, t39320, t40679, t40685);
        let (t46152, t46171) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2225::<F>(t40, t10913, t12939, t4195, t12606, t12862, t12865, t1409, t2244, t2250, t2433, t3966, t40632, t4080, t45872, t607, t73, t9258, t9288, t9427, zeta_threshold);
        let t46190 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2226::<F>(t52, t10913, t12606, t12874, t12877, t1409, t2244, t2250, t2440, t3966, t40647, t4087, t45872, t607, t76, t9258, t9288, t9438, zeta_threshold);
    (t46138, t46140, t46141, t46142, t46143, t46144, t46145, t46152, t46171, t46190)
}

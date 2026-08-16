//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta690 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2135;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2136;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta690(t24995: f64, t34999: f64, t5308: f64, t28813: f64, t6876: f64, t19577: f64, t22574: f64, t33136: f64, t19451: f64, t6535: f64, t28830: f64, t31035: f64, t1390: f64, t19631: f64, t1983: f64, t6878: f64, t25989: f64, t91655: f64, t1845: f64, t5356: f64, t26161: f64, t26162: f64, t26114: f64, t7468: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96805, t96807, t96813, t96815, t96818) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2135(t24995, t34999, t5308, t28813, t6876, t19577, t22574, t33136, t19451, t6535, t28830, t31035);
        let (t96827, t96829, t96833, t96837) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2136(t1390, t19631, t1983, t6878, t25989, t91655, t1845, t5356, t26161, t26162, t26114, t7468);
    (t96805, t96807, t96813, t96815, t96818, t96827, t96829, t96833, t96837)
}

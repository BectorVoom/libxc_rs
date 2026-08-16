//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta181 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk890;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk891;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta181(t300: f64, t4865: f64, t4833: f64, t1687: f64, t1166: f64, t1703: f64, t3411: f64, t1694: f64, t3375: f64, t1157: f64, t1164: f64, t1147: f64, t1156: f64, t4857: f64, t3400: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4866, t4868, t4869) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk890(t300, t4865, t4833, t1687);
        let (t4871, t4873, t4874, t4875, t4877, t4879, t4881, t4882) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk891(t1166, t4869, t1703, t3411, t1694, t3375, t1157, t1164, t1147, t1156, t4857, t3400);
    (t4866, t4868, t4869, t4871, t4873, t4874, t4875, t4877, t4879, t4881, t4882)
}

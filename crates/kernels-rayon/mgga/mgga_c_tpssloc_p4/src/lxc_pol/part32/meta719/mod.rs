//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta719 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2285;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2286;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta719(t55388: f64, t7015: f64, t20173: f64, t28896: f64, t28893: f64, t6534: f64, t1401: f64, t96729: f64, t16524: f64, t26542: f64, t1458: f64, t26135: f64, t3941: f64, t4072: f64, t7467: f64, t28017: f64, t3938: f64, t12524: f64, t28899: f64, t75795: f64, t7769: f64, t5371: f64, t26550: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t100875, t100879, t100883, t100885, t100887, t100890) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2285(t55388, t7015, t20173, t28896, t28893, t6534, t1401, t96729, t16524, t26542, t1458, t26135, t3941);
        let (t100893, t100897, t100899, t100902, t100908, t100915) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2286(t3941, t4072, t7467, t28017, t3938, t12524, t28899, t75795, t7769, t26135, t5371, t16524, t26550);
    (t100875, t100879, t100883, t100885, t100887, t100890, t100893, t100897, t100899, t100902, t100908, t100915)
}

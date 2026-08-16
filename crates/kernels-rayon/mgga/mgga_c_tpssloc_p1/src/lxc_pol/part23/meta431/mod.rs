//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta431 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1267;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1268;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta431(t15438: f64, t19095: f64, t19083: f64, t4993: f64, t18392: f64, t5024: f64, t1226: f64, t22115: f64, t1227: f64, t21776: f64, t248: f64, t3521: f64, t5005: f64, t15737: f64, t18356: f64, t19040: f64, t11738: f64, t22299: f64, t3570: f64, t11728: f64, t22312: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t72248, t72251, t72253, t72255, t72273) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1267(t15438, t19095, t19083, t4993, t18392, t5024, t1226, t22115, t1227, t21776, t248, t3521);
        let (t72285, t72287, t72289, t72293, t72297) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1268(t18392, t5005, t15737, t18356, t19040, t5024, t11738, t22299, t248, t3570, t11728, t22312);
    (t72248, t72251, t72253, t72255, t72273, t72285, t72287, t72289, t72293, t72297)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta425 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1254;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1255;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta425(t10403: f64, t10422: f64, t21525: f64, t18030: f64, t4630: f64, t17884: f64, t4644: f64, t13969: f64, t21502: f64, t3039: f64, t1041: f64, t21550: f64, t135: f64, t21537: f64, t973: f64, t21541: f64, t21545: f64, t13995: f64, t18041: f64, t17659: f64, t21573: f64, t3070: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t70535, t70554, t70573, t70597, t70640) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1254(t10403, t10422, t21525, t18030, t4630, t17884, t4644, t13969, t21502, t3039, t1041, t21550);
        let (t70655, t70660, t70665, t70703, t70711, t70724) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1255(t135, t21537, t973, t21541, t21545, t13995, t18041, t17659, t4644, t10422, t21573, t3070);
    (t70535, t70554, t70573, t70597, t70640, t70655, t70660, t70665, t70703, t70711, t70724)
}

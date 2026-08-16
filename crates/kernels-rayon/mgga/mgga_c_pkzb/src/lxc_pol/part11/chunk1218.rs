//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1218/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1218(t11043: f64, t2899: f64, t5974: f64, t10932: f64, t11025: f64, t11070: f64, t11088: f64, t17852: f64, t2104: f64, t2105: f64, t2106: f64, t21468: f64, t25485: f64, t2901: f64, t29754: f64, t29775: f64, t29950: f64, t29953: f64, t302: f64, t5965: f64, t759: f64, t761: f64, t7725: f64, t7742: f64, t9282: f64, t9314: f64, t9319: f64) -> f64 {
    let t29956 = t2899 * t5974 * t11043;
    let t29970 = -0.42874018118069736972e-3_f64 * t25485 - 0.77173232612525526552e-2_f64 * t21468 * t302 * t29775 * t9319 - 0.51448821741683684368e-2_f64 * t2104 * t17852 * t10932 * t759 * t761 - 0.42874018118069736972e-3_f64 * t2104 * t2105 * t11088 * t2106 - 0.68598428988911579157e-2_f64 * t7725 * t11025 + 0.85748036236139473947e-3_f64 * t29950 + 0.85748036236139473947e-3_f64 * t29953 - 0.17149607247227894789e-2_f64 * t29956 + 0.25724410870841842184e-2_f64 * t7742 * t2105 * t11070 * t5965 - 0.38586616306262763276e-2_f64 * t7742 * t302 * t9282 * t9314 + 0.42874018118069736972e-3_f64 * t2899 * t302 * t29754 * t2901;
    t29970
}

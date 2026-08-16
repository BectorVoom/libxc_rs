//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1218/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1218<F: Float>(t11043: F, t2899: F, t5974: F, t10932: F, t11025: F, t11070: F, t11088: F, t17852: F, t2104: F, t2105: F, t2106: F, t21468: F, t25485: F, t2901: F, t29754: F, t29775: F, t29950: F, t29953: F, t302: F, t5965: F, t759: F, t761: F, t7725: F, t7742: F, t9282: F, t9314: F, t9319: F) -> F {
    let t29956 = t2899 * t5974 * t11043;
    let t29970 = -F::cast_from(0.42874018118069736972e-3_f64) * t25485 - F::cast_from(0.77173232612525526552e-2_f64) * t21468 * t302 * t29775 * t9319 - F::cast_from(0.51448821741683684368e-2_f64) * t2104 * t17852 * t10932 * t759 * t761 - F::cast_from(0.42874018118069736972e-3_f64) * t2104 * t2105 * t11088 * t2106 - F::cast_from(0.68598428988911579157e-2_f64) * t7725 * t11025 + F::cast_from(0.85748036236139473947e-3_f64) * t29950 + F::cast_from(0.85748036236139473947e-3_f64) * t29953 - F::cast_from(0.17149607247227894789e-2_f64) * t29956 + F::cast_from(0.25724410870841842184e-2_f64) * t7742 * t2105 * t11070 * t5965 - F::cast_from(0.38586616306262763276e-2_f64) * t7742 * t302 * t9282 * t9314 + F::cast_from(0.42874018118069736972e-3_f64) * t2899 * t302 * t29754 * t2901;
    t29970
}

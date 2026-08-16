//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1238/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1238(t10792: f64, t10795: f64, t10842: f64, t10878: f64, t1095: f64, t1096: f64, t1916: f64, t1938: f64, t26053: f64, t26062: f64, t26336: f64, t2815: f64, t2816: f64, t2834: f64, t2853: f64, t30221: f64, t30223: f64, t30225: f64, t30227: f64, t3565: f64, t3577: f64, t5825: f64, t5830: f64, t5871: f64, t5897: f64, t702: f64, t9422: f64, t9493: f64) -> f64 {
    let t30440 = t30221 - t30223 + t30225 - t30227 - 24.0_f64 * t5830 * t10878 * t702 + 18.0_f64 * t1938 * t3565 * t2815 + 0.11579025239058625248e4_f64 * t5871 * t10842 * t702 - 6.0_f64 * t5897 * t10792 - 6.0_f64 * t1916 * t2816 * t3577 - 6.0_f64 * t1916 * t1096 * t9493 + 0.96491876992155210402e2_f64 * t5825 * t10795 + 0.96491876992155210402e2_f64 * t1938 * t26336 * t1095 + 0.96491876992155210402e2_f64 * t1938 * t9422 * t2815 - 0.35089341735807877242e1_f64 * t26062 * t2834 + 0.51947577317044391276e2_f64 * t26053 * t2853;
    t30440
}

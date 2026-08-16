//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 689/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk689(t10122: f64, t883: f64, t2325: f64, t882: f64, t12353: f64, t12361: f64, t12367: f64, t3133: f64, t7937: f64, t2268: f64, t12425: f64, t10166: f64, t3129: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12819 = t883 * t10122;
    let t12820 = t2325 * t12819;
    let t12821 = t882 * t12820;
    let t12823 = 0.23712505529730124666e-2_f64 * t12353;
    let t12824 = 0.23712505529730124666e-2_f64 * t12361;
    let t12825 = 0.23712505529730124666e-2_f64 * t12367;
    let t12826 = t7937 * t3133;
    let t12828 = 0.34146007962811379518e0_f64 * t2268 * t12826;
    let t12829 = 0.47425011059460249332e-2_f64 * t12425;
    let t12830 = t10166 * t3129;
    (t12820, t12821, t12823, t12824, t12825, t12826, t12828, t12829, t12830)
}

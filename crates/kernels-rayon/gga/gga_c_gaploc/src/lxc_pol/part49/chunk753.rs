//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 753/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk753(t3133: f64, t7937: f64, t2268: f64, t12425: f64, t10166: f64, t3129: f64, t9074: f64, t12428: f64, t3152: f64, t988: f64, t3340: f64, t894: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12826 = t7937 * t3133;
    let t12828 = 0.34146007962811379518e0_f64 * t2268 * t12826;
    let t12829 = 0.47425011059460249332e-2_f64 * t12425;
    let t12830 = t10166 * t3129;
    let t12831 = t9074 * t12830;
    let t12832 = 0.71137516589190373998e-2_f64 * t12831;
    let t12833 = 0.71137516589190373998e-2_f64 * t12428;
    let t12834 = t3152 * t988;
    let t12836 = 0.28455006635676149599e-1_f64 * t2268 * t12834;
    let t12837 = t894 * t3340;
    (t12826, t12828, t12829, t12830, t12832, t12833, t12834, t12836, t12837)
}

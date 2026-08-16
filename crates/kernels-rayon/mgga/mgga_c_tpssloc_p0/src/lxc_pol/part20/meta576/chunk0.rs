//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2139/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2139(t10199: f64, t2970: f64, t973: f64, t10203: f64, t10254: f64, t10913: f64, t697: f64, t976: f64, t984: f64, t2986: f64, t2990: f64, t10189: f64, t3008: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43028 = t973 * t2970 * t10199;
    let t43038 = t973 * t2970 * t10203;
    let t43043 = t10254 * t10913;
    let t43052 = t697 * t976;
    let t43053 = t43052 * t984;
    let t43055 = t2986 * t43053 * t2990;
    let t43057 = t10189 * t3008;
    (t43028, t43038, t43043, t43052, t43053, t43055, t43057)
}

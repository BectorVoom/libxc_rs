//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2138/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2138(t10224: f64, t2995: f64, t973: f64, t10228: f64, t2960: f64, t10225: f64, t10213: f64, t135: f64, t10218: f64, t10236: f64, t10913: f64, t41961: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42962 = t973 * t10224 * t2995;
    let t42964 = t2960 * t10228;
    let t42968 = t2960 * t10225;
    let t42972 = t135 * t10213;
    let t42974 = t973 * t42972 * t10218;
    let t42985 = t10236 * t10913;
    let t43002 = 220.0_f64 / 81.0_f64 * t41961;
    (t42962, t42964, t42968, t42972, t42974, t42985, t43002)
}

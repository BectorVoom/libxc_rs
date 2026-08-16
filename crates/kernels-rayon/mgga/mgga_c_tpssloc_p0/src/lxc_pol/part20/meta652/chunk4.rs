//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2404/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2404(t49127: f64, t49140: f64, t49154: f64, t49167: f64, t49181: f64, t49194: f64, t49208: f64, t49219: f64, t1556: f64, t2842: f64, t10727: f64, t10702: f64) -> (f64, f64, f64) {
    let t49222 = t49127 + t49140 + t49154 + t49167 + t49181 + t49194 + t49208 + t49219;
    let t49226 = t2842 * t1556;
    let t49228 = 18.0_f64 * t49226 * t10727;
    let t49240 = t10702 * t1556;
    (t49222, t49228, t49240)
}

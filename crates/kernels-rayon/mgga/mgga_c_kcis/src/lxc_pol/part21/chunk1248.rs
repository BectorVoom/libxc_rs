//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1248/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1248(t26854: f64, t7687: f64, t15573: f64, t26731: f64, t2173: f64, t10995: f64, t2836: f64, t93157: f64, t26783: f64, t26781: f64, t26717: f64, t2865: f64, t979: f64, t990: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93606 = t7687 * t26854;
    let t93609 = t15573 * t26731;
    let t93610 = t2173 * t93609;
    let t93620 = t2836 * t10995;
    let t93628 = 0.73697530864197530862e-3_f64 * t93157;
    let t93637 = t15573 * t26783;
    let t93638 = t26781 * t93637;
    let t93653 = t7687 * t26717;
    let t93658 = t979 * t2865 * t990;
    (t93606, t93609, t93610, t93620, t93628, t93637, t93638, t93653, t93658)
}

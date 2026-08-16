//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 809/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk809(t3208: f64, t926: f64, t3206: f64, t3188: f64, t3185: f64, t3224: f64, t6475: f64, t2380: f64, t2428: f64, t3278: f64, t3258: f64, t6514: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8456 = t926 * t3208;
    let t8458 = 0.28582678745379824648e-3_f64 * t3206 * t8456;
    let t8467 = t926 * t3188;
    let t8469 = 0.57165357490759649296e-3_f64 * t3185 * t8467;
    let t8470 = t6475 * t3224;
    let t8472 = 0.57165357490759649296e-3_f64 * t2380 * t8470;
    let t8500 = t2428 * t3278;
    let t8507 = t6514 * t3258;
    (t8456, t8458, t8467, t8469, t8470, t8472, t8500, t8507)
}

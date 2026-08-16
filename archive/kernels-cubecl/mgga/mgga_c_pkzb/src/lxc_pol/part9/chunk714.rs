//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 714/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk714<F: Float>(t482: F, t4936: F, t4932: F, t50: F, t65: F, t4929: F, t4934: F, t4937: F, t4939: F, t4943: F, t4945: F) -> (F, F, F) {
    let t4947 = t482 * t4936;
    let t4950 = t65 * t50 * t4932;
    let t4952 = -F::cast_from(0.34523333333333333333e1_f64) * t4929 + F::cast_from(0.23015555555555555556e1_f64) * t4934 - F::cast_from(0.26851481481481481482e1_f64) * t4937 - F::cast_from(0.93932222222222222223e0_f64) * t4939 + F::cast_from(0.73355e-1_f64) * t4943 - F::cast_from(0.14671e0_f64) * t4945 - F::cast_from(0.17116166666666666667e0_f64) * t4947 - F::cast_from(0.36793333333333333333e0_f64) * t4950;
    (t4947, t4950, t4952)
}

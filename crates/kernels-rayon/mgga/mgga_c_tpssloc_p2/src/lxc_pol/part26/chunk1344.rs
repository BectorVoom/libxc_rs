//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1344/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1344(t2363: f64, t24932: f64, t27888: f64, t671: f64, t7266: f64, t83946: f64, t83948: f64, t83952: f64, t83956: f64, t83958: f64, t83960: f64, t83962: f64, t83964: f64, t83966: f64, t83968: f64, t85428: f64, t85573: f64, t85577: f64, t9416: f64) -> f64 {
    let t85613 = 6.0_f64 * t2363 * t24932 + 6.0_f64 * t2363 * t27888 + 6.0_f64 * t671 * t85428 + 2.0_f64 * t7266 * t9416 + t83946 + t83948 + t83952 + t83956 + t83958 + t83960 + t83962 + t83964 + t83966 + t83968 + t85573 + 6.0_f64 * t85577;
    t85613
}

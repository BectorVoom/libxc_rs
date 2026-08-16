//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta577 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2140;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta577(t2986: f64, t2990: f64, t43057: f64, t10325: f64, t2987: f64, t3008: f64, t4509: f64, t13797: f64, t984: f64, t10216: f64, t343: f64, t9288: f64, t10236: f64, t10427: f64, t13969: f64, t3130: f64, t10432: f64, t3039: f64, t10943: f64, t135: f64, t973: f64, t3152: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43059, t43061, t43065, t43069, t43070, t43071) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2140(t2986, t2990, t43057, t10325, t2987, t3008, t4509, t13797, t984, t10216, t343, t9288);
        let (t43075, t43094, t43097, t43103, t43110) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2141(t10236, t9288, t10427, t13969, t3130, t10432, t3039, t10943, t135, t973, t3152, t698);
    (t43059, t43061, t43065, t43069, t43070, t43071, t43075, t43094, t43097, t43103, t43110)
}

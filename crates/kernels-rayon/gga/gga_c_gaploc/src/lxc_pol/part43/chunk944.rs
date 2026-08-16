//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 944/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk944(t8862: f64, t9780: f64, t1052: f64, t29646: f64, t10105: f64, t1960: f64, t3689: f64, t874: f64) -> (f64, f64, f64, f64) {
    let t44238 = 4.0_f64 * t8862 * t9780;
    let t44239 = t29646 * t1052;
    let t44242 = 2.0_f64 * t1960 * t1052 * t10105;
    let t46849 = t3689 * t874;
    (t44238, t44239, t44242, t46849)
}

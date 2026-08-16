//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2446/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2446(t2955: f64, t3158: f64, t10383: f64, t964: f64, t1020: f64, t10508: f64, t248: f64, t3121: f64, t10868: f64, t820: f64, t3070: f64, t3072: f64) -> (f64, f64, f64, f64, f64) {
    let t43155 = t2955 * t3158;
    let t43157 = t964 * t10383;
    let t43161 = t1020 * t248 * t10508 * t3121;
    let t43198 = t820 * t10868;
    let t43200 = t3070 * t43198 * t3072;
    (t43155, t43157, t43161, t43198, t43200)
}

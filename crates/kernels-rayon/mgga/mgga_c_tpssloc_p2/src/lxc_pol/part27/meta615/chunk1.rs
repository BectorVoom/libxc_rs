//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2092/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2092(t23476: f64, t23479: f64, t6722: f64, t23563: f64, t6740: f64, t6747: f64, t23422: f64, t3139: f64, t10922: f64, t6717: f64, t10993: f64, t10981: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t83134 = t6722 * t23476 * t23479;
    let t83138 = t6740 * t23563;
    let t83139 = t83138 * t6747;
    let t83153 = t23422 * t3139;
    let t83157 = t6717 * t10922;
    let t83159 = t6717 * t10993;
    let t83165 = t6717 * t10981;
    (t83134, t83138, t83139, t83153, t83157, t83159, t83165)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 896/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk896(t21158: f64, t21193: f64, t932: f64, t10813: f64, t21114: f64, t21089: f64, t2932: f64, t10542: f64, t10545: f64, t21120: f64, t21124: f64, t21128: f64, t21132: f64, t21136: f64, t21140: f64, t21142: f64, t21144: f64, t21147: f64, t21150: f64, t21153: f64, t21156: f64) -> (f64, f64, f64, f64) {
    let t21194 = t21158 + t21193;
    let t21195 = t21194 * t932;
    let t21198 = t21114 * t10813;
    let t21207 = t21089 * t2932;
    let t21222 = 0.16557e0_f64 * t21120 - 0.60384999999999999999e0_f64 * t21124 + 0.181155e1_f64 * t21128 - 0.36793333333333333333e-1_f64 * t21132 - 0.82785e-1_f64 * t21136 - 0.49671e0_f64 * t21140 - 0.3883875e1_f64 * t21142 + 0.247573125e0_f64 * t21144 - t10542 - t10545 - 0.33547222222222222222e0_f64 * t21147 + 0.12077e1_f64 * t21150 - 0.181155e1_f64 * t21153 - 0.301925e0_f64 * t21156;
    (t21195, t21198, t21207, t21222)
}

//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 520/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk520(t2702: f64, t626: f64, t1045: f64, t1055: f64, t184: f64, t188: f64, t2671: f64, t2679: f64, t622: f64, t634: f64) -> (f64, f64) {
    let t2703 = t626 * t2702;
    let t2706 = 0.65854491829355115987e0_f64 * t2671 * t188 - 0.65854491829355115987e0_f64 * t1045 * t634 - 0.65854491829355115987e0_f64 * t622 * t1055 + 0.13170898365871023197e1_f64 * t184 * t2679 - 0.65854491829355115987e0_f64 * t184 * t2703;
    (t2703, t2706)
}

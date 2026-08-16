//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2912/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2912(t300: f64, t52282: f64, t52324: f64, t52377: f64, t52433: f64, t52477: f64, t52520: f64, t52817: f64, t52856: f64, t11502: f64, t4724: f64, t981: f64) -> (f64, f64) {
    let t52860 = t300 * (t52282 + t52324 + t52377 + t52433 + t52477 + t52520 + t52817 + t52856);
    let t52863 = 0.11696447245269292414e1_f64 * t981 * t4724 * t11502;
    (t52860, t52863)
}

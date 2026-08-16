//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 912/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk912(t31812: f64, t8471: f64, t886: f64, t2718: f64, t2769: f64, t231: f64, t836: f64, t1949: f64, t7048: f64, t8650: f64, t25386: f64, t8485: f64) -> (f64, f64, f64, f64, f64) {
    let t31814 = t31812 * t8471 * t886;
    let t31817 = t2769 * t2718;
    let t31819 = t8471 * t836 * t231;
    let t31820 = t31817 * t31819;
    let t31824 = t8650 * t1949 * t7048;
    let t31827 = t25386 * t8485;
    (t31814, t31817, t31820, t31824, t31827)
}

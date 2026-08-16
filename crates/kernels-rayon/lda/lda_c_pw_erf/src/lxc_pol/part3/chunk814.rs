//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 814/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk814(t1859: f64, t443: f64, t1710: f64, t770: f64, t1712: f64, t774: f64, t1878: f64, t450: f64, t1724: f64, t155: f64, t436: f64, t1870: f64, t1872: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5618 = t1859 * t443;
    let t5621 = t770 * t1710;
    let t5630 = t774 * t1712;
    let t5633 = t1878 * t450;
    let t5636 = t774 * t1724;
    let t5639 = t155 * t436;
    let t5641 = t1870 * t5639 * t1872;
    (t5618, t5621, t5630, t5633, t5636, t5639, t5641)
}

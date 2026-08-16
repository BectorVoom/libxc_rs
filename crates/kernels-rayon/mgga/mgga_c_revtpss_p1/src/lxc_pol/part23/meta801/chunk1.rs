//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2629/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2629(t18616: f64, t2798: f64, t686: f64, t72: f64, t61532: f64, t836: f64, t2782: f64, t39597: f64, t6022: f64, t10529: f64, t10952: f64, t18525: f64, t2482: f64, t5977: f64) -> (f64, f64, f64, f64) {
    let t62587 = t2798 * t18616 * t72 * t686;
    let t62589 = t61532 * t836;
    let t62591 = t2782 * t39597 * t62589;
    let t62593 = t6022 * t836;
    let t62595 = t2782 * t10529 * t62593;
    let t62601 = t2482 * t10952 * t5977 * t18525 * t72 * t686;
    (t62587, t62591, t62595, t62601)
}

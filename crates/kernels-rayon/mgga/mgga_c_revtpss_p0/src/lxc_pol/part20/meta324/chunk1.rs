//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1237/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1237(t12975: f64, t480: f64, t3667: f64, t3678: f64, t1236: f64, t371: f64, t676: f64, t1235: f64, t12627: f64, t225: f64) -> (f64, f64, f64, f64, f64) {
    let t12976 = t12975 * t480;
    let t12979 = t3667 * t3678;
    let t12984 = t371 * t676 * t1236;
    let t12985 = t1235 * t12984;
    let t12987 = t12627 * t225;
    (t12976, t12979, t12984, t12985, t12987)
}

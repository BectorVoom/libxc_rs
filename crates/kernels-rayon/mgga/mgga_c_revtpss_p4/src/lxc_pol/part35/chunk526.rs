//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 526/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk526(t1903: f64, t72: f64, t686: f64, t3915: f64, t1882: f64, t555: f64, t4086: f64, t543: f64, t2782: f64, t1883: f64, t4101: f64, t225: f64, t3999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5721 = t1903 * t72;
    let t5722 = t5721 * t686;
    let t5723 = t3915 * t5722;
    let t5735 = t555 * t1882;
    let t5737 = t4086 * t5735 * t543;
    let t5738 = t2782 * t5737;
    let t5740 = t1883 * t72;
    let t5741 = t5740 * t686;
    let t5742 = t4101 * t5741;
    let t5744 = t225 * t3999;
    (t5721, t5722, t5723, t5735, t5737, t5738, t5740, t5741, t5742, t5744)
}

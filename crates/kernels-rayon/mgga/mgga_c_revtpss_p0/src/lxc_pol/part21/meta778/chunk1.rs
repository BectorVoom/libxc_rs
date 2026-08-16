//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2771/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2771(t150: f64, t190: f64, t50034: f64, t40076: f64, t40079: f64, t40194: f64, t40198: f64, t50884: f64, t50887: f64, t50889: f64, t50891: f64, t50892: f64, t50894: f64, t50897: f64, t50898: f64, t50899: f64, t50900: f64, t50902: f64, t50905: f64) -> (f64, f64) {
    let t50907 = t150 * t50034 * t190;
    let t50908 = t50884 + t50887 - t50889 + t50891 + t50892 + t50894 + t50897 + t50898 + t40076 - t40079 + t40194 + t40198 + t50899 - t50900 - t50902 + t50905 + t50907;
    (t50907, t50908)
}

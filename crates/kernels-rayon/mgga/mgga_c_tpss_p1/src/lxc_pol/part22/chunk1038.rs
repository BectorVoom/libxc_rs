//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1038/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1038(t11231: f64, t3908: f64, t912: f64, t2596: f64, t3907: f64, t10954: f64, t10956: f64, t10963: f64, t10965: f64, t10968: f64, t10970: f64, t10972: f64, t11103: f64, t11123: f64, t11146: f64, t11149: f64, t11155: f64, t11160: f64, t11211: f64, t11215: f64, t11218: f64, t11221: f64) -> (f64, f64, f64) {
    let t11232 = t11231 * t3908;
    let t11234 = 0.34631718211362927518e2_f64 * t912 * t11232;
    let t11235 = t3907 * t2596;
    let t11237 = 0.35089341735807877242e1_f64 * t912 * t11235;
    let t11238 = t10954 - t10956 + t10963 + t10965 + t10968 + t10970 + t10972 + t11103 + t11123 - t11146 - t11149 + t11155 - t11160 - t11211 + t11215 - t11218 - t11221;
    (t11234, t11237, t11238)
}

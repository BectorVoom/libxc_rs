//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1205/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1205(t2782: f64, t5804: f64, t1856: f64, t5802: f64, t1899: f64, t2786: f64, t5796: f64, t1095: f64, t17514: f64, t17519: f64, t17521: f64, t17621: f64, t17624: f64, t17630: f64, t17638: f64, t1955: f64, t1956: f64, t1971: f64, t2834: f64, t2849: f64, t2853: f64, t5484: f64, t5825: f64, t5831: f64, t5838: f64, t5890: f64, t5903: f64, t721: f64, t7248: f64, t7255: f64, t7258: f64, t7261: f64, t7293: f64, t7299: f64, t7308: f64, t7475: f64, t7494: f64) -> (f64, f64, f64) {
    let t20918 = t2782 * t5804;
    let t20921 = 0.1551780387578202009e4_f64 * t5802 * t20918 * t1856;
    let t20924 = 0.16081979498692535067e2_f64 * t1899 * t2786 * t5796;
    let t20957 = -t20921 - t20924 + 0.6207121550312808036e4_f64 * t17514 * t7248 + 0.19964560303604640732e6_f64 * t17519 * t1095 * t17521 * t5831 - 0.35089341735807877242e1_f64 * t7494 * t5890 - 0.35089341735807877242e1_f64 * t17621 * t2834 - 0.70178683471615754484e1_f64 * t5903 * t7255 - 0.35089341735807877242e1_f64 * t5903 * t7258 - 0.31168546390226634765e3_f64 * t17624 * t7261 + 0.51947577317044391277e2_f64 * t17630 * t2853 - 0.35089341735807877242e1_f64 * t1955 * t7475 * t721 - 0.35089341735807877242e1_f64 * t1955 * t2849 * t1971 - 0.12304822629859687989e5_f64 * t17638 * t7308 * t5484 - 0.31168546390226634765e3_f64 * t5838 * t7299 * t1956 + 18.0_f64 * t5825 * t7293;
    (t20921, t20924, t20957)
}

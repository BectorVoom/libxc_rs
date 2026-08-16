//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1205/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1205<F: Float>(t2782: F, t5804: F, t1856: F, t5802: F, t1899: F, t2786: F, t5796: F, t1095: F, t17514: F, t17519: F, t17521: F, t17621: F, t17624: F, t17630: F, t17638: F, t1955: F, t1956: F, t1971: F, t2834: F, t2849: F, t2853: F, t5484: F, t5825: F, t5831: F, t5838: F, t5890: F, t5903: F, t721: F, t7248: F, t7255: F, t7258: F, t7261: F, t7293: F, t7299: F, t7308: F, t7475: F, t7494: F) -> (F, F, F) {
    let t20918 = t2782 * t5804;
    let t20921 = F::cast_from(0.1551780387578202009e4_f64) * t5802 * t20918 * t1856;
    let t20924 = F::cast_from(0.16081979498692535067e2_f64) * t1899 * t2786 * t5796;
    let t20957 = -t20921 - t20924 + F::cast_from(0.6207121550312808036e4_f64) * t17514 * t7248 + F::cast_from(0.19964560303604640732e6_f64) * t17519 * t1095 * t17521 * t5831 - F::cast_from(0.35089341735807877242e1_f64) * t7494 * t5890 - F::cast_from(0.35089341735807877242e1_f64) * t17621 * t2834 - F::cast_from(0.70178683471615754484e1_f64) * t5903 * t7255 - F::cast_from(0.35089341735807877242e1_f64) * t5903 * t7258 - F::cast_from(0.31168546390226634765e3_f64) * t17624 * t7261 + F::cast_from(0.51947577317044391277e2_f64) * t17630 * t2853 - F::cast_from(0.35089341735807877242e1_f64) * t1955 * t7475 * t721 - F::cast_from(0.35089341735807877242e1_f64) * t1955 * t2849 * t1971 - F::cast_from(0.12304822629859687989e5_f64) * t17638 * t7308 * t5484 - F::cast_from(0.31168546390226634765e3_f64) * t5838 * t7299 * t1956 + F::cast_from(18.0_f64) * t5825 * t7293;
    (t20921, t20924, t20957)
}

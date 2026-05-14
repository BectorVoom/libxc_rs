//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 833/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk833<F: Float>(t16060: F, t5701: F, t1889: F, t4001: F, t12185: F, t16073: F, t5709: F, t1377: F, t5713: F, t1380: F, t5477: F, t16082: F, t498: F, t16078: F, t16069: F, t12125: F, t12129: F, t12131: F, t12145: F, t12149: F, t1368: F) -> (F, F, F) {
    let t16874 = t5701 * t16060;
    let t16877 = t1889 * t4001;
    let t16878 = t12185 * t16877;
    let t16881 = t5709 * t16073;
    let t16884 = t5713 * t1377;
    let t16885 = t5477 * t1380;
    let t16886 = t16884 * t16885;
    let t16889 = t5709 * t16082;
    let t16892 = t5713 * t498;
    let t16893 = t16892 * t16078;
    let t16896 = t5701 * t16069;
    let t16899 = -t12125 / 288.0 + t12131 / 216.0 + t12145 / 144.0 + t12129 - t12149 / 432.0 - t1368 * t16874 / 36.0 + t1368 * t16878 / 144.0 + t1368 * t16881 / 48.0 + t1368 * t16886 / 72.0 - t1368 * t16889 / 144.0 + t1368 * t16893 / 36.0 + t1368 * t16896 / 216.0;
    (t16884, t16892, t16899)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 994/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk994(t15498: f64, t15499: f64, t42944: f64, t590: f64, t23000: f64, t33308: f64, t9889: f64, t43007: f64, t739: f64, t1991: f64, t43107: f64, t1890: f64) -> (f64, f64, f64, f64, f64) {
    let t43830 = 0.61348681526273199482e1_f64 * t15498 * t15499 * t42944 * t590;
    let t43832 = t23000 * t33308 * t9889;
    let t43833 = 0.11502877786176224903e1_f64 * t43832;
    let t43834 = t739 * t43007;
    let t43836 = t1991 * t43834 * t590;
    let t43838 = t739 * t43107;
    let t43841 = 0.1022478025437886658e1_f64 * t1991 * t43838 * t590;
    let t43842 = t1890 * t43007;
    (t43830, t43833, t43836, t43841, t43842)
}

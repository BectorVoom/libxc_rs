//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1072/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1072(t123: f64, t1358: f64, t46873: f64, t488: f64, t13740: f64, t2312: f64, t42580: f64, t42582: f64, t42584: f64, t42588: f64, t42591: f64, t46859: f64, t46862: f64, t46865: f64, t46871: f64) -> f64 {
    let t46877 = 0.31616674039640166221e-2_f64 * t1358 * t46873 * t123 * t488;
    let t46878 = t2312 * t13740;
    let t46880 = -t42580 + 0.11856252764865062333e-2_f64 * t46859 - 0.35568758294595186999e-2_f64 * t46862 + 0.23712505529730124666e-2_f64 * t46865 + t46871 + t42582 + 0.23712505529730124666e-2_f64 * t42584 - t42588 - t42591 - t46877 - 0.11856252764865062333e-2_f64 * t46878;
    t46880
}

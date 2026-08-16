//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1034/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1034(t42594: f64, t42597: f64, t42601: f64, t42602: f64, t42603: f64, t42604: f64, t42605: f64, t42606: f64, t46878: f64, t46884: f64, t46887: f64, t46889: f64, t46892: f64, t46896: f64, t46900: f64, t46908: f64, t46912: f64, t46913: f64) -> f64 {
    let t50949 = -0.23712505529730124666e-2_f64 * t46878 - 0.23712505529730124666e-2_f64 * t46884 - 0.23712505529730124666e-2_f64 * t46887 - t42594 + t42597 + t42601 + t42602 - t42603 + 0.23712505529730124666e-2_f64 * t46889 + 0.23712505529730124666e-2_f64 * t46892 + 0.68292015925622759038e0_f64 * t46896 - 0.3414600796281137952e0_f64 * t46900 + t42604 + t42605 - t42606 - 0.1138200265427045984e0_f64 * t46908 + t46912 + 0.7588001769513639893e-1_f64 * t46913;
    t50949
}

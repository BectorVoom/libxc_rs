//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1034/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1034<F: Float>(t42594: F, t42597: F, t42601: F, t42602: F, t42603: F, t42604: F, t42605: F, t42606: F, t46878: F, t46884: F, t46887: F, t46889: F, t46892: F, t46896: F, t46900: F, t46908: F, t46912: F, t46913: F) -> F {
    let t50949 = -F::cast_from(0.23712505529730124666e-2_f64) * t46878 - F::cast_from(0.23712505529730124666e-2_f64) * t46884 - F::cast_from(0.23712505529730124666e-2_f64) * t46887 - t42594 + t42597 + t42601 + t42602 - t42603 + F::cast_from(0.23712505529730124666e-2_f64) * t46889 + F::cast_from(0.23712505529730124666e-2_f64) * t46892 + F::cast_from(0.68292015925622759038e0_f64) * t46896 - F::cast_from(0.3414600796281137952e0_f64) * t46900 + t42604 + t42605 - t42606 - F::cast_from(0.1138200265427045984e0_f64) * t46908 + t46912 + F::cast_from(0.7588001769513639893e-1_f64) * t46913;
    t50949
}

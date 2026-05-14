//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 538/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk538<F: Float>(t2568: F, t6930: F, t242: F, t6181: F, t6184: F, t6881: F, t6885: F, t6889: F, t6893: F, t6897: F, t6901: F, t6905: F) -> (F, F, F) {
    let t6931 = t2568 * t6930;
    let t6932 = t242 * t6931;
    let t6940 = t6881 / 4.0 + t6181 + t6885 / 6.0 + t6889 - t6893 / 2.0 + t6184 + t6897 / 3.0 + 2.0 * t6901 - t6905;
    (t6931, t6932, t6940)
}

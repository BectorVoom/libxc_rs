//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 463/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk463<F: Float>(t184: F, t4888: F, t21: F, t1078: F, t1079: F, t920: F, t2321: F, t992: F, t1131: F, t231: F, t1137: F, t1526: F, t2319: F, t2320: F, t342: F, t343: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4889 = t4888 * t184;
    let t4890 = t4889 * t21;
    let t4893 = t1078 * t1078;
    let t4894 = t4893 * t184;
    let t4895 = t4894 * t21;
    let t4898 = t1079 * t920;
    let t4906 = t2321 * t992;
    let t4910 = t231 * t1131;
    let t4914 = t1137 - t2319 - t1526 * t2320 * t4906 / 12.0 - t342 * t343 * t4910 / 4.0;
    (t4889, t4890, t4893, t4894, t4895, t4898, t4906, t4910, t4914)
}

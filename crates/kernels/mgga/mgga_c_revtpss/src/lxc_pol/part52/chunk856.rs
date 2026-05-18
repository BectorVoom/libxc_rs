//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 856/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk856<F: Float>(t1973: F, t3201: F, t1058: F, t7114: F, t1020: F, t7131: F, t1971: F, t3104: F, t351: F, t25516: F, t3114: F, t3057: F, t7143: F) -> (F, F, F, F, F, F) {
    let t25560 = F::new(0.95275595817932748827e-4) * t1973 * t3201;
    let t25564 = t7114 * t1058;
    let t25569 = t1020 * t7131;
    let t25576 = t1971 * t3104;
    let t25577 = t351 * t25576;
    let t25580 = t3114 * t25516;
    let t25591 = t3057 * t7143;
    (t25560, t25564, t25569, t25577, t25580, t25591)
}

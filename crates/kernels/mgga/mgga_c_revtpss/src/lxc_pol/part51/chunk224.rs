//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 224/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk224<F: Float>(t972: F, t973: F, t300: F, t311: F, t912: F, t938: F, t941: F, t946: F, t955: F, t961: F, t965: F, t315: F) -> (F, F, F, F) {
    let t974 = t972 * t973;
    let t978 = t300 * (-F::new(0.310907e-1) * t941 * t311 + F::new(1.0) * t946 * t955 + t912 - t938 - F::new(0.19751673498613801407e-1) * t961 + F::new(0.5848223622634646207e0) * t965 * t974);
    let t980 = F::new(0.19751673498613801407e-1) * t300 * t961;
    let t981 = t300 * t315;
    (t974, t978, t980, t981)
}

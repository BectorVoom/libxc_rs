//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1018/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1018<F: Float>(t34886: F, t35017: F, t3: F, t1918: F, t34003: F, t34006: F, t34009: F, t34011: F, t34014: F, t34481: F, t34483: F, t34485: F, t573: F, t8616: F, t8975: F, param_d: F) -> (F, F, F, F) {
    let t35018 = t34886 + t35017;
    let t35019 = t3 * t35018;
    let t35027 = param_d * t35018;
    let t35034 = F::new(3.0) * t1918 * t8975 + t35027 * t573 + t34003 + t34006 + t34009 + t34011 + t34014 + F::new(6.0) * t34481 + F::new(12.0) * t34483 + F::new(6.0) * t34485 + t8616;
    (t35018, t35019, t35027, t35034)
}

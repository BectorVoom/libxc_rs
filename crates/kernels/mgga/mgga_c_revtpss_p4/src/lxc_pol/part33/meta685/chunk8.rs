//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2271/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2271<F: Float>(t105818: F, t105822: F, t105826: F, t105830: F, t105834: F, t105837: F, t105839: F, t105841: F, t105843: F, t109282: F, t109288: F, t2170: F, t22556: F, t22559: F, t22565: F, t22568: F, t5802: F, t6945: F, t7696: F, t8245: F) -> F {
    let t113039 = F::new(6.0) * t2170 * t22556 + F::new(12.0) * t2170 * t22559 + F::new(6.0) * t2170 * t22565 + F::new(3.0) * t2170 * t22568 + F::new(12.0) * t5802 * t8245 + F::new(6.0) * t6945 * t7696 + t105818 + t105822 + t105826 + t105830 + t105834 + t105837 + t105839 + t105841 + t105843 + t109282 + t109288;
    t113039
}

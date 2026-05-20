//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2227/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2227<F: Float>(t109041: F, t109043: F, t109045: F, t109047: F, t109049: F, t109052: F, t109054: F, t109058: F, t109060: F, t109063: F, t109074: F, t109078: F, t109081: F, t1518: F, t18242: F, t2322: F, t27060: F, t29337: F, t29432: F, t30963: F, t4254: F, t5921: F, t651: F, t7586: F) -> F {
    let t111762 = -F::new(4.0) * t1518 * t29337 * t651 - F::new(2.0) * t18242 * t7586 - F::new(4.0) * t2322 * t30963 - F::new(2.0) * t27060 * t5921 - F::new(2.0) * t29432 * t5921 - F::new(4.0) * t30963 * t4254 - t109041 - t109043 - t109045 - t109047 + t109049 + t109052 - t109054 - t109058 - t109060 - t109063 + t109074 + t109078 - t109081;
    t111762
}

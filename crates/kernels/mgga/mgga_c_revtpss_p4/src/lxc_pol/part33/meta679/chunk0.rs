//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2212/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2212<F: Float>(t109269: F, t28199: F, t25082: F, t27153: F, t33651: F, t6941: F, t7331: F, t5795: F, t7950: F, t7953: F, t1916: F, t28265: F) -> (F, F, F, F, F, F) {
    let t109271 = F::new(4.0) * t109269 * t28199;
    let t109274 = F::new(6.0) * t25082 * t33651 * t27153;
    let t109282 = F::new(6.0) * t6941 * t7331;
    let t109288 = F::new(12.0) * t5795 * t7950;
    let t109291 = F::new(6.0) * t5795 * t7953;
    let t109293 = F::new(12.0) * t1916 * t28265;
    (t109271, t109274, t109282, t109288, t109291, t109293)
}

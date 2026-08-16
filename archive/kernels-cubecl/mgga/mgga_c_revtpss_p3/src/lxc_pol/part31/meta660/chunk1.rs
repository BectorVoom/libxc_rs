//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2236/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2236<F: Float>(t5517: F, t651: F, t7741: F, t101417: F, t2014: F, t7900: F, t109035: F, t109038: F, t109039: F, t109041: F, t109043: F, t109045: F, t109047: F, t109049: F, t109052: F, t109054: F, t109058: F, t109060: F, t1518: F, t2322: F, t27830: F, t29986: F, t30116: F, t33602: F, t4254: F, t4293: F, t649: F) -> F {
    let t109063 = F::cast_from(4.0_f64) * t651 * t5517 * t7741;
    let t109074 = F::cast_from(6.0_f64) * t2014 * t101417 * t7900;
    let t109075 = -F::cast_from(4.0_f64) * t1518 * t27830 * t651 - F::cast_from(4.0_f64) * t2322 * t30116 - t29986 * t649 - F::cast_from(4.0_f64) * t30116 * t4254 - F::cast_from(4.0_f64) * t33602 * t4293 - t109035 - t109038 - t109039 - t109041 - t109043 - t109045 - t109047 + t109049 + t109052 - t109054 - t109058 - t109060 - t109063 + t109074;
    t109075
}

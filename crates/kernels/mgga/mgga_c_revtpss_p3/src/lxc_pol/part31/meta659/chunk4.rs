//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2234/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2234<F: Float>(t28167: F, t86753: F, t8717: F, t13648: F, t2014: F, t7934: F, t29589: F, t7235: F, t13426: F, t7742: F, t18227: F, t28063: F, t4248: F) -> (F, F, F, F, F, F) {
    let t109035 = F::new(6.0) * t28167 * t8717 * t86753;
    let t109038 = F::new(2.0) * t2014 * t7934 * t13648;
    let t109039 = t7235 * t29589;
    let t109041 = F::new(4.0) * t13426 * t7742;
    let t109043 = F::new(4.0) * t18227 * t7742;
    let t109045 = F::new(4.0) * t4248 * t28063;
    (t109035, t109038, t109039, t109041, t109043, t109045)
}

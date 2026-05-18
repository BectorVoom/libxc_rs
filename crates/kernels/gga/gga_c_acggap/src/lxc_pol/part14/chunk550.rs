//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 550/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk550<F: Float>(t3088: F, t4184: F, t1642: F, t3378: F, t1539: F, t4166: F, t1160: F, t1630: F, t3077: F, t150: F, t2934: F, t119: F) -> (F, F, F, F, F) {
    let t4185 = t3088 * t4184;
    let t4188 = F::new(0.13170898365871023197e1) * t3378 * t1642;
    let t4189 = t4166 * t1539;
    let t4191 = F::new(0.13170898365871023197e1) * t1160 * t4189;
    let t4192 = t3077 * t1630;
    let t4197 = t150 * t2934;
    let t4198 = t119 * t4197;
    (t4185, t4188, t4191, t4192, t4198)
}

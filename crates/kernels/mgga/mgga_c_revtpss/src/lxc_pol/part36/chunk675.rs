//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 675/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk675<F: Float>(t116: F, t5883: F, t117: F, t5920: F, t1916: F, t1918: F, t572: F, t573: F, t6941: F, t624: F, t112: F, t655: F, t68: F, t1949: F, t212: F, t780: F) -> (F, F, F, F, F, F, F, F) {
    let t6945 = t116 * t5883;
    let t6948 = t117 * t5920;
    let t6951 = 6.0 * t1916 * t1918 + 6.0 * t572 * t6945 + 3.0 * t572 * t6948 + t573 * t6941;
    let t6971 = 8.0 / 3.0 * t624;
    let t6996 = t624 * t112;
    let t6997 = t6996 / 3.0;
    let t6998 = t68 * t655;
    let t7014 = t212 * t1949;
    let t7015 = t7014 * t780;
    (t6945, t6948, t6951, t6971, t6997, t6998, t7014, t7015)
}

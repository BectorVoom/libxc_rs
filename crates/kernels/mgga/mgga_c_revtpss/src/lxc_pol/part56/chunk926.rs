//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 926/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk926<F: Float>(t531: F, t8594: F, t7238: F, t2014: F, t7235: F, t8600: F, t2322: F, t8461: F, t4254: F, t1310: F, t8460: F, t651: F) -> (F, F, F, F, F, F, F, F) {
    let t32098 = t531 * t8594;
    let t32099 = t32098 * t7238;
    let t32101 = F::new(3.0) * t2014 * t32099;
    let t32102 = t7235 * t8600;
    let t32106 = t2322 * t8461;
    let t32107 = F::new(2.0) * t32106;
    let t32108 = t4254 * t8461;
    let t32109 = F::new(2.0) * t32108;
    let t32110 = t1310 * t8460;
    let t32111 = t651 * t32110;
    (t32098, t32099, t32101, t32102, t32107, t32109, t32110, t32111)
}

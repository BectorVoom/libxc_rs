//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1072/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1072<F: Float>(t32041: F, t36019: F, t8306: F, t32181: F, t36475: F, t38086: F, t2385: F, t310: F, t464: F, t9369: F, t2131: F, t2147: F, t309: F, t9413: F) -> (F, F, F, F, F, F) {
    let t38215 = t32041 * t8306 * t36019;
    let t38224 = t32181 * t38086 * t36475;
    let t38226 = t310 * t2385;
    let t38228 = F::new(0.13170898365871023197e1) * t38226 * t464;
    let t38232 = F::new(0.13170898365871023197e1) * t310 * t9369;
    let t38241 = F::new(0.34694512752820797848e1) * t2131 * t2147 * t9413 * t309;
    (t38215, t38224, t38226, t38228, t38232, t38241)
}

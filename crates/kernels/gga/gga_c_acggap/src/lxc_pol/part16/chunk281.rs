//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 281/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk281<F: Float>(t183: F, t848: F, t1004: F, t453: F, t377: F, t457: F, t310: F, t460: F, t452: F, t864: F, t1035: F, t180: F, t322: F, t407: F, t1160: F, t159: F, t441: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1226 = 0.65854491829355115987e0 * t848 * t183;
    let t1228 = 0.13170898365871023197e1 * t1004 * t453;
    let t1229 = t377 * t457;
    let t1231 = t310 * t460;
    let t1233 = t452 * t864;
    let t1235 = 0.13170898365871023197e1 * t1035 * t1233;
    let t1236 = t180 * t322;
    let t1237 = t1236 * t407;
    let t1238 = t1160 * t1237;
    let t1240 = t159 * t441;
    (t1226, t1228, t1229, t1231, t1233, t1235, t1237, t1238, t1240)
}

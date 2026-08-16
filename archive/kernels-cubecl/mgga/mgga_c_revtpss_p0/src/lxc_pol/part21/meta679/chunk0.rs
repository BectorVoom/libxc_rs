//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2491/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2491<F: Float>(t13069: F, t3704: F, t12941: F, t3708: F, t12948: F, t13058: F, t12937: F, t3172: F, t3711: F, t13080: F, t5384: F, t1231: F, t12898: F) -> (F, F, F, F, F, F) {
    let t44278 = t13069 * t3704;
    let t44280 = t3708 * t12941;
    let t44283 = t13058 * t12948;
    let t44286 = t3711 * t3172 * t12937;
    let t44289 = t5384 * t3172 * t13080;
    let t44291 = t1231 * t12898;
    (t44278, t44280, t44283, t44286, t44289, t44291)
}

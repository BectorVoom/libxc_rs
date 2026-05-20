//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2991/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2991<F: Float>(t2349: F, t656: F, t10227: F, t97: F, t10241: F, t105: F, t4273: F, t588: F, t2289: F, t4288: F, t13455: F, t625: F) -> (F, F, F, F, F, F) {
    let t49774 = t656 * t2349;
    let t49777 = t97 * t10227;
    let t49787 = t105 * t10241;
    let t49804 = F::new(20.0) * t97 * t4273 * t588;
    let t49817 = t2289 * t4288;
    let t49819 = t625 * t13455;
    (t49774, t49777, t49787, t49804, t49817, t49819)
}

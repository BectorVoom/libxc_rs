//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2327/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2327<F: Float>(t231: F, t268: F, t2798: F, t793: F, t836: F, t215: F, t2722: F, t2645: F, t4366: F, t10529: F, t2782: F, t14545: F, t251: F) -> (F, F, F, F, F, F, F) {
    let t39581 = t2798 * t268 * t793 * t836 * t231;
    let t39583 = t215 * t2722;
    let t39586 = t2798 * t268 * t39583 * t231;
    let t39588 = t4366 * t2645;
    let t39590 = t2782 * t10529 * t39588;
    let t39595 = t2798 * t268 * t215 * t2645 * t231;
    let t39597 = t14545 * t251;
    (t39581, t39583, t39586, t39588, t39590, t39595, t39597)
}

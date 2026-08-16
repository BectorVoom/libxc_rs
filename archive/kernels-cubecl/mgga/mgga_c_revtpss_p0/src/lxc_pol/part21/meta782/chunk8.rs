//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2808/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2808<F: Float>(t1558: F, t2482: F, t2801: F, t2815: F, t10547: F, t14606: F, t10538: F, t14605: F, t49180: F, t14586: F, t2645: F, t10529: F, t2782: F) -> (F, F, F, F) {
    let t51598 = t2482 * t2815 * t1558 * t2801;
    let t51600 = t14606 * t10547;
    let t51603 = t49180 * t14605 * t10538;
    let t51604 = F::cast_from(0.34697458558045176417e-2_f64) * t51603;
    let t51608 = t14586 * t2645;
    let t51610 = t2782 * t10529 * t51608;
    (t51598, t51600, t51604, t51610)
}

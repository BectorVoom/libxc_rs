//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1402/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1402<F: Float>(t268: F, t39644: F, t556: F, t561: F, t8779: F, t786: F, t9656: F, t4146: F, t1892: F, t9646: F, t9648: F, t1904: F, t47567: F) -> (F, F, F, F, F) {
    let t47601 = F::cast_from(0.11638313500518478545e-4_f64) * t39644 * t556 * t561 * t8779 * t268;
    let t47603 = t786 * t556 * t9656;
    let t47671 = t4146 * t4146;
    let t47672 = F::cast_from(1.0_f64) / t47671;
    let t47764 = t9646 * t1892 * t9648;
    let t47772 = t47567 * t1904;
    (t47601, t47603, t47672, t47764, t47772)
}

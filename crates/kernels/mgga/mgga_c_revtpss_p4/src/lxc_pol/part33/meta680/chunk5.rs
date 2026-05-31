//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2219/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2219<F: Float>(t10309: F, t104317: F, t108807: F, t108810: F, t108813: F, t1470: F, t2121: F, t2123: F, t28093: F, t28105: F, t28109: F, t28147: F, t28154: F, t29388: F, t29513: F, t29551: F, t7576: F, t7579: F, t8144: F) -> F {
    let t111577 = F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t29388 * t28105 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t29388 * t28109 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t108807 * t2123 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t108810 * t2123 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t108813 * t2123 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t28154 * t104317 + F::cast_from(20.0_f64) * t10309 * t1470 * t2121 * t28147 - t29513 * t7576 / F::cast_from(6.0_f64) - t29513 * t7579 / F::cast_from(6.0_f64) - t28093 * t8144 / F::cast_from(3.0_f64) + t29551 * t7576 / F::cast_from(3.0_f64) + t29551 * t7579 / F::cast_from(3.0_f64);
    t111577
}

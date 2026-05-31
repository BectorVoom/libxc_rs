//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3823/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3823<F: Float>(t73493: F, t13625: F, t13674: F, t1868: F, t1907: F, t198: F, t33596: F, t39799: F, t39807: F, t39813: F, t4139: F, t47059: F, t49647: F, t530: F, t73418: F, t73474: F, t73477: F, t73482: F, t73488: F) -> (F, F) {
    let t73494 = F::cast_from(0.36622894612013090108e-3_f64) * t73493;
    let t73495 = -F::cast_from(24.0_f64) * t13625 * t1907 * t198 * t33596 * t530 + F::cast_from(12.0_f64) * t13674 * t4139 * t73488 + F::cast_from(6.0_f64) * t1868 * t4139 * t49647 + t39799 + t39807 - t39813 + t47059 + t73418 + t73474 + t73477 - t73482 - t73494;
    (t73494, t73495)
}

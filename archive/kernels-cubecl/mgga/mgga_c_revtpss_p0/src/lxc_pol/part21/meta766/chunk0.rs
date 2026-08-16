//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2716/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2716<F: Float>(t10428: F, t4305: F, t2609: F, t4186: F, t706: F, t10436: F, t4311: F, t14426: F, t72: F, t757: F, t18875: F, t2403: F, t2411: F, t2832: F, t39786: F, t39791: F, t39795: F, t39799: F, t39807: F, t39813: F) -> (F, F, F, F, F) {
    let t49978 = t10428 * t4305;
    let t49979 = F::cast_from(12.0_f64) * t49978;
    let t49981 = t706 * t2609 * t4186;
    let t49982 = F::cast_from(12.0_f64) * t49981;
    let t49983 = t4311 * t10436;
    let t49984 = F::cast_from(12.0_f64) * t49983;
    let t49986 = t14426 * t72 * t757;
    let t49987 = F::cast_from(0.54934341918019635162e-3_f64) * t49986;
    let t49988 = -F::cast_from(9.0_f64) * t18875 * t2403 * t2411 * t2832 - t39786 - t39791 - t39795 + t39799 + t39807 - t39813 + t49979 + t49982 + t49984 - t49987;
    (t49979, t49982, t49984, t49987, t49988)
}

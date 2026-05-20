//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1781/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1781<F: Float>(t1196: F, t45187: F, t45190: F, t90357: F, t90602: F, t90629: F, t90631: F, t90634: F, t90636: F, t90640: F, t90644: F, t90855: F, t90857: F, t90860: F, t90863: F) -> (F, F) {
    let t90867 = F::cast_from(0.91082604192152556044e5_f64) * t1196 * t45187 * t90357 * t45190;
    let t90868 = -t90602 - t90629 - t90631 + t90634 - t90636 + t90640 + t90644 + t90855 + t90857 - t90860 - t90863 - t90867;
    (t90867, t90868)
}

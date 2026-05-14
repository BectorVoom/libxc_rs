//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 928/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk928<F: Float>(t2156: F, t2993: F, t1306: F, t135: F, t273: F, t7314: F, t7413: F, t7415: F, t7417: F, t7446: F, t7485: F, t7491: F, t7493: F, t7504: F, t7552: F, t7554: F, t7557: F, t7559: F, t7562: F, t7564: F, t7566: F, t7570: F, t7573: F, t7888: F, t803: F, t805: F) -> (F, F) {
    let t7892 = t2993 * t2156;
    let t7896 = t135 * t273 * t7888 * t805 - 2.0 * t1306 * t7892 * t803 + t7314 + t7413 + t7415 + t7417 + t7446 - t7485 + t7491 + t7493 - t7504 - t7552 + t7554 + t7557 - t7559 - t7562 - t7564 + t7566 - t7570 - t7573;
    (t7892, t7896)
}

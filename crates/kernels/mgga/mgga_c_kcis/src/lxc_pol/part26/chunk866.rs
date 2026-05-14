//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 866/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk866<F: Float>(t21628: F, t21790: F, t509: F, t552: F, t557: F, t303: F, t1014: F, t7195: F, t1489: F, t6927: F, t1396: F, t11826: F, t1464: F, t1497: F, t1495: F, t4123: F) -> (F, F, F, F, F, F, F, F) {
    let t21791 = t21628 + t21790;
    let t21792 = t509 * t21791;
    let t21793 = t21792 * t552;
    let t21794 = t21793 * t557;
    let t21795 = t303 * t21794;
    let t21797 = t1014 * t7195;
    let t21799 = t6927 * t1489;
    let t21800 = t1396 * t21799;
    let t21801 = t11826 * t21800;
    let t21802 = t1464 * t21801;
    let t21804 = t6927 * t1497;
    let t21805 = t1495 * t21804;
    let t21806 = t4123 * t21805;
    (t21791, t21792, t21795, t21797, t21799, t21802, t21804, t21806)
}

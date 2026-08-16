//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1316/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1316(t21628: f64, t21790: f64, t509: f64, t552: f64, t557: f64, t303: f64, t1014: f64, t7195: f64, t1489: f64, t6927: f64, t1396: f64, t11826: f64) -> (f64, f64, f64, f64, f64) {
    let t21791 = t21628 + t21790;
    let t21792 = t509 * t21791;
    let t21793 = t21792 * t552;
    let t21794 = t21793 * t557;
    let t21795 = t303 * t21794;
    let t21797 = t1014 * t7195;
    let t21799 = t6927 * t1489;
    let t21800 = t1396 * t21799;
    let t21801 = t11826 * t21800;
    (t21791, t21795, t21797, t21799, t21801)
}

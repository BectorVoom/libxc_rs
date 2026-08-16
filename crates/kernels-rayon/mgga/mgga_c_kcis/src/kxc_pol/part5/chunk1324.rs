//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1324/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1324(t21655: f64, t5653: f64, t4170: f64, t16771: f64, t1307: f64, t7309: f64, t4160: f64, t1459: f64, t7104: f64, t303: f64, t1489: f64, t6922: f64) -> (f64, f64, f64, f64) {
    let t21894 = t5653 * t21655;
    let t21895 = t4170 * t21894;
    let t21896 = t16771 * t21895;
    let t21898 = t7309 * t1307;
    let t21899 = t4170 * t21898;
    let t21900 = t4160 * t21899;
    let t21902 = t1459 * t7104;
    let t21903 = t303 * t21902;
    let t21905 = t6922 * t1489;
    (t21896, t21900, t21903, t21905)
}

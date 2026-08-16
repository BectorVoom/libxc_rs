//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1186/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1186(t19763: f64, t3338: f64, t3337: f64, t10707: f64, t6693: f64, t5053: f64, t5083: f64, t10745: f64, t6720: f64, t19756: f64, t5077: f64, t5076: f64) -> (f64, f64, f64, f64, f64) {
    let t19870 = t3338 * t19763;
    let t19871 = t3337 * t19870;
    let t19873 = t10707 * t6693;
    let t19875 = t5083 * t5053;
    let t19877 = t10745 * t6720;
    let t19879 = t5077 * t19756;
    let t19880 = t5076 * t19879;
    (t19871, t19873, t19875, t19877, t19880)
}

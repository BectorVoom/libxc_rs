//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1398/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1398(t18079: f64, t21110: f64, t1610: f64, t6944: f64, t4440: f64, t21073: f64, t6159: f64, t21078: f64, t6151: f64, t531: f64, t7429: f64, t833: f64) -> (f64, f64, f64, f64, f64) {
    let t23073 = t18079 * t21110;
    let t23076 = t6944 * t1610;
    let t23077 = t4440 * t23076;
    let t23080 = t6159 * t21073;
    let t23083 = t6151 * t21078;
    let t23086 = t7429 * t531;
    let t23087 = t23086 * t833;
    (t23073, t23077, t23080, t23083, t23087)
}

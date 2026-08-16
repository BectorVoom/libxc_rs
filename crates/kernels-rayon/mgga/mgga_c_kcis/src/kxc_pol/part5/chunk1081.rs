//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1081/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1081(t6276: f64, t829: f64, t4546: f64, t3210: f64, t3200: f64, t4555: f64, t4554: f64, t2861: f64, t6488: f64, t6493: f64, t13192: f64, t4549: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18508 = t6276 * t829;
    let t18509 = t4546 * t18508;
    let t18510 = t3210 * t18509;
    let t18511 = t3200 * t18510;
    let t18513 = t4555 * t18508;
    let t18514 = t3210 * t18513;
    let t18515 = t4554 * t18514;
    let t18517 = t2861 * t6488;
    let t18521 = t2861 * t6493;
    let t18523 = t13192 * t4549;
    (t18508, t18511, t18515, t18517, t18521, t18523)
}

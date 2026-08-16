//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1151/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1151(t1773: f64, t3219: f64, t9546: f64, t1021: f64, t1092: f64, t1767: f64, t3228: f64, t1022: f64, t9589: f64, t2855: f64, t4818: f64, t3227: f64) -> (f64, f64, f64, f64, f64) {
    let t14611 = t1773 * t3219;
    let t14612 = t9546 * t14611;
    let t14613 = t1021 * t14612;
    let t14614 = t1092 * t14613;
    let t14616 = t1767 * t3228;
    let t14617 = t1022 * t14616;
    let t14618 = t9589 * t14617;
    let t14619 = t1092 * t14618;
    let t14622 = t2855 * t4818;
    let t14623 = t3227 * t14622;
    (t14611, t14614, t14616, t14619, t14623)
}

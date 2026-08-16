//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1255/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1255(t20974: f64, t5653: f64, t4162: f64, t4160: f64, t1497: f64, t6281: f64, t11898: f64, t4170: f64, t833: f64, t5662: f64, t5627: f64, t5632: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20975 = t5653 * t20974;
    let t20976 = t4162 * t20975;
    let t20977 = t4160 * t20976;
    let t20979 = t6281 * t1497;
    let t20980 = t11898 * t20979;
    let t20981 = t4170 * t20980;
    let t20982 = t4160 * t20981;
    let t20984 = t6281 * t833;
    let t20985 = t5662 * t20984;
    let t20986 = t4170 * t20985;
    let t20987 = t4160 * t20986;
    let t20989 = t5632 * t5627;
    (t20977, t20979, t20982, t20984, t20987, t20989)
}

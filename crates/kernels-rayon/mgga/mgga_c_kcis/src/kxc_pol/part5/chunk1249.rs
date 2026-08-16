//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1249/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1249(t20883: f64, t4170: f64, t4160: f64, t20882: f64, t5662: f64, t5661: f64, t4142: f64, t7030: f64, t11913: f64, t7101: f64, t3728: f64, t7207: f64) -> (f64, f64, f64, f64, f64) {
    let t20884 = t4170 * t20883;
    let t20885 = t4160 * t20884;
    let t20887 = t5662 * t20882;
    let t20888 = t4170 * t20887;
    let t20889 = t5661 * t20888;
    let t20892 = t4142 * t7030;
    let t20894 = t11913 * t7101;
    let t20898 = t3728 * t7207;
    (t20885, t20889, t20892, t20894, t20898)
}

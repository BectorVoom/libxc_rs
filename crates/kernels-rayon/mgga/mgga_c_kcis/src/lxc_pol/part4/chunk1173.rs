//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1173/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1173(t374: f64, t983: f64, t5078: f64, t3463: f64, t5048: f64, t1196: f64, t5169: f64, t1195: f64, t5067: f64, t1187: f64, t10752: f64, t380: f64) -> (f64, f64, f64, f64, f64) {
    let t14857 = t374 * t983;
    let t14858 = t14857 * t5078;
    let t14860 = t3463 * t983;
    let t14861 = t14860 * t5048;
    let t14863 = t5169 * t1196;
    let t14865 = t1195 * t5067;
    let t14866 = t1187 * t14865;
    let t14868 = t380 * t10752;
    (t14858, t14861, t14863, t14866, t14868)
}

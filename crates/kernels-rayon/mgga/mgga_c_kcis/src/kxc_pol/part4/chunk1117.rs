//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1117/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1117(t1121: f64, t4772: f64, t1022: f64, t3227: f64, t1092: f64, t1767: f64, t3316: f64, t2861: f64, t4820: f64, t4825: f64, t10245: f64, t4813: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14092 = t4772 * t1121;
    let t14093 = t1022 * t14092;
    let t14094 = t3227 * t14093;
    let t14095 = t1092 * t14094;
    let t14097 = t1767 * t3316;
    let t14098 = t1022 * t14097;
    let t14099 = t3227 * t14098;
    let t14100 = t1092 * t14099;
    let t14102 = t2861 * t4820;
    let t14103 = 0.66327777777777777776e-2_f64 * t14102;
    let t14104 = t2861 * t4825;
    let t14106 = t10245 * t4813;
    (t14092, t14095, t14097, t14100, t14102, t14103, t14104, t14106)
}

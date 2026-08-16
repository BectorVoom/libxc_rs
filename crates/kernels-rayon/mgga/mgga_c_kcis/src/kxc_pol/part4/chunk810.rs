//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 810/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk810(t1094: f64, t1795: f64, t1172: f64, t1195: f64, t1816: f64, t382: f64, t1813: f64, t3477: f64, t3338: f64, t4984: f64, t3337: f64, t1196: f64, t1809: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5082 = t1795 * t1094;
    let t5083 = t5082 * sigma0;
    let t5084 = t5083 * t1172;
    let t5086 = t1195 * t1816;
    let t5087 = t382 * t5086;
    let t5089 = t3477 * t1813;
    let t5091 = t3338 * t4984;
    let t5092 = t3337 * t5091;
    let t5094 = t1809 * t1196;
    (t5082, t5083, t5084, t5086, t5087, t5089, t5091, t5092, t5094)
}

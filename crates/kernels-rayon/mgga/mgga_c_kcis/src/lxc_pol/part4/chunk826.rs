//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 826/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk826(t1268: f64, t5341: f64, t1239: f64, t1844: f64, t1240: f64, t1269: f64, t1857: f64, t3248: f64, t3638: f64, t3644: f64, t3658: f64, t4987: f64, t4990: f64, t4997: f64, t5001: f64, t5003: f64, t5007: f64, t5011: f64, t5015: f64, t5017: f64, t5021: f64, t5023: f64, t5028: f64, t5031: f64, t5282: f64) -> (f64, f64, f64) {
    let t5342 = t5341 * t1268;
    let t5345 = t1844 * t1239;
    let t5357 = 0.890445125e-2_f64 * t3644 * t5282 + 0.66725e-1_f64 * t1240 * t5282 - t3658 - 0.30952962962962962963e-2_f64 * t3248 - 0.11607361111111111111e-2_f64 * t4987 + 0.11607361111111111111e-2_f64 * t4990 + 0.23214722222222222222e-2_f64 * t4997 + 0.11607361111111111111e-2_f64 * t5001 + 0.77382407407407407407e-3_f64 * t5003 - 0.30952962962962962963e-2_f64 * t5007 - 0.66725e-1_f64 * t1240 * t5342 - 0.66725e-1_f64 * t5345 * t1269 - 0.66725e-1_f64 * t3638 * t1857 + 0.11607361111111111111e-2_f64 * t5011 - 0.30952962962962962963e-2_f64 * t5015 - 0.11607361111111111111e-2_f64 * t5017 + 0.46429444444444444443e-2_f64 * t5021 + 0.77382407407407407407e-3_f64 * t5023 - 0.17411041666666666666e-2_f64 * t5028 + 0.11607361111111111111e-2_f64 * t5031;
    (t5342, t5345, t5357)
}

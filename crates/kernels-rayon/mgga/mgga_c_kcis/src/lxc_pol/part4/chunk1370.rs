//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1370/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1370(t5981: f64, t8931: f64, t1507: f64, t16005: f64, t16009: f64, t16030: f64, t16403: f64, t16417: f64, t16548: f64, t17649: f64, t17656: f64, t17669: f64, t17673: f64, t17676: f64, t17685: f64, t1991: f64, t2018: f64, t2429: f64, t3782: f64, t3816: f64, t4193: f64, t4202: f64, t5133: f64, t5459: f64, t5482: f64, t5947: f64, t5958: f64) -> f64 {
    let t17686 = t8931 * t5981;
    let t17688 = 0.26531111111111111111e0_f64 * t5133 * t17649 + 0.9286875e-2_f64 * t4193 * t1991 + 0.1857375e-1_f64 * t1507 * t5482 + 0.9286875e-2_f64 * t17656 * t5459 - 0.371475e-1_f64 * t5958 * t16403 + 0.24765e-1_f64 * t5958 * t16548 + 0.9286875e-2_f64 * t5947 * t16005 + 0.46434375e-2_f64 * t5947 * t16009 - 0.1857375e-1_f64 * t4202 * t16030 - 0.9286875e-2_f64 * t5947 * t17669 + 0.10612444444444444444e0_f64 * t2429 * t17673 + 0.5895802469135802469e-2_f64 * t17676 + 0.123825e-1_f64 * t2018 * t3816 + 0.46434375e-2_f64 * t2018 * t3782 + 0.1857375e-1_f64 * t4202 * t16417 - t17685 - 0.88437037037037037036e-1_f64 * t17686;
    t17688
}

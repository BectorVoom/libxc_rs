//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1329/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1329(t1401: f64, t5805: f64, t5808: f64, t833: f64, t1962: f64, t2645: f64, t4035: f64, t5526: f64, t1419: f64, t3841: f64, t5804: f64, t2642: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17065 = 0.28111840756657074598e-1_f64 * t1401 * t5805;
    let t17066 = t5808 * t833;
    let t17069 = t1962 * t2645;
    let t17072 = t4035 * t5526;
    let t17073 = t17072 * t1419;
    let t17076 = t5804 * t3841;
    let t17079 = t1962 * t2642;
    (t17065, t17066, t17069, t17073, t17076, t17079)
}

//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1100/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1100(t1666: f64, t2937: f64, t2940: f64, t4682: f64, t930: f64, t951: f64, t2981: f64, t4685: f64, t2985: f64, t2989: f64, t1680: f64, t9650: f64) -> (f64, f64, f64, f64, f64) {
    let t13864 = t1666 * t2937;
    let t13866 = 2.0_f64 * t13864 * t2940;
    let t13867 = t4682 * t930;
    let t13869 = 2.0_f64 * t13867 * t951;
    let t13871 = 1.0_f64 * t4685 * t2981;
    let t13872 = t1666 * t2985;
    let t13874 = 0.16081824322151104822e2_f64 * t13872 * t2989;
    let t13876 = 1.0_f64 * t9650 * t1680;
    (t13866, t13869, t13871, t13874, t13876)
}

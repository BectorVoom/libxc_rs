//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 921/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk921(t14026: f64, t962: f64, t971: f64, t1680: f64, t2939: f64, t2986: f64, t2980: f64, t4722: f64, t1679: f64, t9770: f64, t9768: f64, t4690: f64, t9804: f64) -> (f64, f64, f64, f64, f64) {
    let t14028 = t962 * t14026 * t971;
    let t14033 = t1680 * t2939;
    let t14035 = 6.0_f64 * t2986 * t14033;
    let t14036 = t4722 * t2980;
    let t14038 = 0.16081824322151104822e2_f64 * t2986 * t14036;
    let t14039 = t1679 * t9770;
    let t14040 = t14039 * t2939;
    let t14042 = 0.51725014705706168417e3_f64 * t9768 * t14040;
    let t14044 = 4.0_f64 * t9804 * t4690;
    (t14028, t14035, t14038, t14042, t14044)
}

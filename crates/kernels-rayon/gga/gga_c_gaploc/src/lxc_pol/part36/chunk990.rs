//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 990/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk990(t8775: f64, t9842: f64, t41231: f64, t41234: f64, t41237: f64, t41244: f64, t2021: f64, t43572: f64, t5974: f64, t10817: f64, t9972: f64, t1445: f64, t3209: f64, t813: f64, t8528: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43774 = 0.11916829983950142223e0_f64 * t8775 * t9842;
    let t43775 = 0.63904876589867916127e-1_f64 * t41231;
    let t43776 = 0.59584149919750711116e-1_f64 * t41234;
    let t43777 = 0.29792074959875355558e-1_f64 * t41237;
    let t43778 = 0.63904876589867916127e-1_f64 * t41244;
    let t43781 = 0.25025342966295298669e1_f64 * t2021 * t43572 * t5974;
    let t43783 = 0.50050685932590597338e1_f64 * t10817 * t9972;
    let t43787 = 0.92023022289409799224e1_f64 * t813 * t1445 * t8528 * t3209;
    (t43774, t43775, t43776, t43777, t43778, t43781, t43783, t43787)
}

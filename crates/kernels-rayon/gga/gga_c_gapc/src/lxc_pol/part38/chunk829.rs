//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 829/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk829(t3321: f64, t7927: f64, t3320: f64, t1084: f64, t8686: f64, t1089: f64, t9636: f64, t9639: f64, t9642: f64, t9646: f64, t9649: f64, t9653: f64, t9656: f64, t9659: f64, t9662: f64, t9665: f64, t9668: f64) -> (f64, f64, f64) {
    let t9670 = t7927 * t3321;
    let t9671 = t3320 * t9670;
    let t9673 = t1084 * t8686;
    let t9674 = t9673 * t1089;
    let t9676 = -0.50602213541666666669e-5_f64 * t9636 - 0.86880925264517213544e-4_f64 * t9639 + 0.14480154210752868924e-5_f64 * t9642 + 0.21116891557347933848e-6_f64 * t9646 - 0.11594181388521408695e-4_f64 * t9649 + 0.2813674965076916843e-8_f64 * t9653 + 0.2813674965076916843e-8_f64 * t9656 - 0.27801896084645508334e-2_f64 * t9659 + 0.12163329537032409896e-2_f64 * t9662 - 0.27801896084645508334e-2_f64 * t9665 + 0.9275345110817126956e-4_f64 * t9668 - 0.50027140879067581468e-8_f64 * t9671 - 0.2579202051320507106e-5_f64 * t9674;
    (t9670, t9673, t9676)
}

//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 829/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk829<F: Float>(t3321: F, t7927: F, t3320: F, t1084: F, t8686: F, t1089: F, t9636: F, t9639: F, t9642: F, t9646: F, t9649: F, t9653: F, t9656: F, t9659: F, t9662: F, t9665: F, t9668: F) -> (F, F, F) {
    let t9670 = t7927 * t3321;
    let t9671 = t3320 * t9670;
    let t9673 = t1084 * t8686;
    let t9674 = t9673 * t1089;
    let t9676 = -F::cast_from(0.50602213541666666669e-5_f64) * t9636 - F::cast_from(0.86880925264517213544e-4_f64) * t9639 + F::cast_from(0.14480154210752868924e-5_f64) * t9642 + F::cast_from(0.21116891557347933848e-6_f64) * t9646 - F::cast_from(0.11594181388521408695e-4_f64) * t9649 + F::cast_from(0.2813674965076916843e-8_f64) * t9653 + F::cast_from(0.2813674965076916843e-8_f64) * t9656 - F::cast_from(0.27801896084645508334e-2_f64) * t9659 + F::cast_from(0.12163329537032409896e-2_f64) * t9662 - F::cast_from(0.27801896084645508334e-2_f64) * t9665 + F::cast_from(0.9275345110817126956e-4_f64) * t9668 - F::cast_from(0.50027140879067581468e-8_f64) * t9671 - F::cast_from(0.2579202051320507106e-5_f64) * t9674;
    (t9670, t9673, t9676)
}

//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 755/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk755<F: Float>(t325: F, t8992: F, t2817: F, t3321: F, t7927: F, t3320: F, t1084: F, t8686: F, t1089: F, t9636: F, t9639: F, t9642: F, t9646: F, t9649: F, t9653: F, t9656: F, t9659: F, t9662: F, t9665: F) -> (F, F, F) {
    let t9667 = t325 * t8992;
    let t9668 = t9667 * t2817;
    let t9670 = t7927 * t3321;
    let t9671 = t3320 * t9670;
    let t9673 = t1084 * t8686;
    let t9674 = t9673 * t1089;
    let t9676 = -0.50602213541666666669e-5 * t9636 - 0.86880925264517213544e-4 * t9639 + 0.14480154210752868924e-5 * t9642 + 0.21116891557347933848e-6 * t9646 - 0.11594181388521408695e-4 * t9649 + 0.2813674965076916843e-8 * t9653 + 0.2813674965076916843e-8 * t9656 - 0.27801896084645508334e-2 * t9659 + 0.12163329537032409896e-2 * t9662 - 0.27801896084645508334e-2 * t9665 + 0.9275345110817126956e-4 * t9668 - 0.50027140879067581468e-8 * t9671 - 0.2579202051320507106e-5 * t9674;
    (t9670, t9673, t9676)
}

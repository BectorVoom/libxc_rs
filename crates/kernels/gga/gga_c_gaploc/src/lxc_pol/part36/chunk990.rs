//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 990/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk990<F: Float>(t8775: F, t9842: F, t41231: F, t41234: F, t41237: F, t41244: F, t2021: F, t43572: F, t5974: F, t10817: F, t9972: F, t1445: F, t3209: F, t813: F, t8528: F) -> (F, F, F, F, F, F, F, F) {
    let t43774 = F::new(0.11916829983950142223e0) * t8775 * t9842;
    let t43775 = F::new(0.63904876589867916127e-1) * t41231;
    let t43776 = F::new(0.59584149919750711116e-1) * t41234;
    let t43777 = F::new(0.29792074959875355558e-1) * t41237;
    let t43778 = F::new(0.63904876589867916127e-1) * t41244;
    let t43781 = F::new(0.25025342966295298669e1) * t2021 * t43572 * t5974;
    let t43783 = F::new(0.50050685932590597338e1) * t10817 * t9972;
    let t43787 = F::new(0.92023022289409799224e1) * t813 * t1445 * t8528 * t3209;
    (t43774, t43775, t43776, t43777, t43778, t43781, t43783, t43787)
}

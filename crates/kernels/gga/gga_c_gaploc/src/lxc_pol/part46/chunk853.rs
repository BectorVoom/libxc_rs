//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 853/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk853<F: Float>(t43490: F, t6066: F, t6111: F, t10914: F, t10915: F, t8775: F, t9842: F, t41231: F, t41234: F, t41237: F, t41244: F, t2021: F, t43572: F, t5974: F, t10817: F, t9972: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43768 = t6111 * t6066 * t43490;
    let t43771 = t10914 * t10915 * t43490;
    let t43774 = 0.11916829983950142223e0 * t8775 * t9842;
    let t43775 = 0.63904876589867916127e-1 * t41231;
    let t43776 = 0.59584149919750711116e-1 * t41234;
    let t43777 = 0.29792074959875355558e-1 * t41237;
    let t43778 = 0.63904876589867916127e-1 * t41244;
    let t43781 = 0.25025342966295298669e1 * t2021 * t43572 * t5974;
    let t43783 = 0.50050685932590597338e1 * t10817 * t9972;
    (t43768, t43771, t43774, t43775, t43776, t43777, t43778, t43781, t43783)
}

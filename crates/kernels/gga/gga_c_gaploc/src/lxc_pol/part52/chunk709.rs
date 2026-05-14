//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 709/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk709<F: Float>(t33565: F, t7372: F, t33294: F, t9810: F, t10628: F, t549: F, t6111: F, t24505: F, t2684: F, t9438: F, t3295: F, t8802: F, t9800: F, t13052: F, t1966: F, t28673: F) -> (F, F, F, F, F, F, F) {
    let t43679 = t33565 * t7372;
    let t43681 = t33294 * t9810;
    let t43715 = t6111 * t549 * t10628;
    let t43718 = t2684 * t9438 * t24505;
    let t43756 = t9800 * t8802 * t3295;
    let t43758 = t1966 * t13052;
    let t43760 = t28673 * t13052;
    (t43679, t43681, t43715, t43718, t43756, t43758, t43760)
}

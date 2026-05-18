//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 984/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk984<F: Float>(t23477: F, t42945: F, t4820: F, t10667: F, t123: F, t883: F, t2684: F, t2685: F, t10628: F, t549: F, t6111: F, t24505: F, t9438: F) -> (F, F, F, F, F) {
    let t43708 = F::new(0.23833659967900284446e0) * t23477 * t4820 * t42945;
    let t43710 = t10667 * t123 * t883;
    let t43712 = t2684 * t2685 * t43710;
    let t43715 = t6111 * t549 * t10628;
    let t43716 = F::new(0.11916829983950142223e0) * t43715;
    let t43718 = t2684 * t9438 * t24505;
    (t43708, t43710, t43712, t43716, t43718)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1047/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1047<F: Float>(t25994: F, t652: F, t2314: F, t7468: F, t25965: F, t25969: F, t25973: F, t25975: F, t25977: F, t25979: F, t25982: F, t25987: F, t25991: F, t25993: F, t4028: F, t4034: F, t650: F, t6539: F, t7472: F, t7670: F) -> F {
    let t25996 = F::cast_from(2.0_f64) * t652 * t25994;
    let t25998 = F::cast_from(2.0_f64) * t2314 * t7468;
    let t25999 = -F::cast_from(2.0_f64) * t25965 * t652 - F::cast_from(2.0_f64) * t4028 * t6539 - F::cast_from(2.0_f64) * t4034 * t7472 - t650 * t7670 - t25969 - t25973 - t25975 - t25977 - t25979 - t25982 + t25987 - t25991 - t25993 - t25996 - t25998;
    t25999
}

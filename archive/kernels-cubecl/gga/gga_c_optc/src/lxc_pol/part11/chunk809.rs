//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 809/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk809<F: Float>(t4570: F, t8414: F, t8428: F, t5126: F, t8617: F, t8611: F, t4229: F, t4535: F) -> (F, F, F, F, F) {
    let t14909 = t8414 * t4570;
    let t14914 = t8428 * t4570;
    let t14984 = t8617 * t5126;
    let t14992 = t8611 * t5126;
    let t15008 = t4535 * t4229;
    (t14909, t14914, t14984, t14992, t15008)
}

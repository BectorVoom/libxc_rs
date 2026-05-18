//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1251/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1251<F: Float>(t3638: F, t3949: F, t8459: F, t11239: F, t1476: F, t11512: F, t14541: F, t1459: F, t1649: F, t3635: F, t8419: F, t11683: F, t22971: F, t22973: F, t3737: F) -> (F, F, F, F, F) {
    let t35700 = t8459 * t3638 * t3949;
    let t35702 = t1476 * t11239;
    let t35706 = t14541 * t1459 * t11512 * t1649;
    let t35708 = t8419 * t3635;
    let t35720 = t3737 * t22971 * t11683 * t22973;
    (t35700, t35702, t35706, t35708, t35720)
}

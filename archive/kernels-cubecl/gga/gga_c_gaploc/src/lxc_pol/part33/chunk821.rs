//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 821/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk821<F: Float>(t6507: F, t7893: F, t161: F, t2760: F, t1353: F, t1359: F, t3394: F, t488: F, t447: F, t986: F) -> (F, F, F, F, F) {
    let t7894 = t6507 * t7893;
    let t7897 = t2760 * t161;
    let t7898 = t7897 * t1353;
    let t7901 = t1359 * t3394;
    let t7902 = t7901 * t488;
    let t7905 = t986 * t447;
    (t7894, t7898, t7901, t7902, t7905)
}

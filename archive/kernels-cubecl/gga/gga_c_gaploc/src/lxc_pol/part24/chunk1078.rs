//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1078/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1078<F: Float>(t22623: F, t8502: F, t1980: F, t8520: F, t296: F, t8720: F, t1: F, t787: F, t2021: F, t8774: F, t1022: F, t5514: F) -> (F, F, F, F, F) {
    let t25070 = t22623 * t8502;
    let t25177 = t1980 * t8520;
    let t25191 = t296 * t8720;
    let t25193 = t787 * t25191 * t1;
    let t25198 = t2021 * t8774;
    let t25260 = t5514 * t1022;
    (t25070, t25177, t25193, t25198, t25260)
}

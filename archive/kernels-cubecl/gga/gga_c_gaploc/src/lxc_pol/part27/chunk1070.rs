//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1070/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1070<F: Float>(t5514: F, t935: F, t1858: F, t2530: F, t2021: F, t7530: F, t1854: F, t20901: F, t5679: F, t6110: F, t5580: F, t7426: F) -> (F, F, F, F, F, F) {
    let t23292 = t5514 * t935;
    let t23296 = t1858 * t2530;
    let t23309 = t2021 * t7530;
    let t23310 = t20901 * t1854;
    let t23335 = t5679 * t6110;
    let t23344 = t5580 * t7426;
    (t23292, t23296, t23309, t23310, t23335, t23344)
}

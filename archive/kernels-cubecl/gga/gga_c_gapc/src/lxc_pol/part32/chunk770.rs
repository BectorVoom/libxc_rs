//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 770/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk770<F: Float>(t1743: F, t9019: F, t1749: F, t3060: F, t3127: F, t3132: F, t5285: F, t1881: F, t512: F, t178: F, t173: F, t7216: F) -> (F, F, F, F, F, F, F) {
    let t9020 = t1743 * t9019;
    let t9021 = t9020 * t1749;
    let t9023 = t3060 * t3127;
    let t9024 = t9023 * t1749;
    let t9026 = t5285 * t3132;
    let t9027 = t9026 * t1749;
    let t9029 = t1881 * t512;
    let t9030 = t178 * t9029;
    let t9031 = t173 * t7216;
    (t9020, t9021, t9024, t9027, t9029, t9030, t9031)
}

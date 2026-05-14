//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 592/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk592<F: Float>(t5148: F, t8621: F, t5259: F, t8649: F, t570: F, t7778: F, t305: F, t2064: F, t551: F) -> (F, F, F, F, F) {
    let t8971 = t5148 * t8621;
    let t8973 = t5259 * t8649;
    let t8997 = t7778 * t570;
    let t8998 = t305 * t8997;
    let t9000 = t2064 * t551;
    (t8971, t8973, t8997, t8998, t9000)
}

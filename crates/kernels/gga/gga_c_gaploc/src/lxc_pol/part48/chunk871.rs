//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 871/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk871<F: Float>(t299: F, t3431: F, t550: F, t43027: F, t13624: F, t1841: F, t2536: F, t734: F, t1022: F) -> (F, F, F, F, F) {
    let t44878 = t299 * t3431;
    let t44879 = t550 * t44878;
    let t44883 = F::new(0.1281754371690370714e-2) * t43027;
    let t44887 = F::new(0.85450291446024714263e-3) * t1841 * t2536 * t13624 * t734;
    let t44888 = t1022 * t3431;
    (t44878, t44879, t44883, t44887, t44888)
}

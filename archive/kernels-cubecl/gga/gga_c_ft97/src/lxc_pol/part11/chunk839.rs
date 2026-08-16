//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 839/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk839<F: Float>(t1866: F, t37320: F, t446: F, t1643: F, t1755: F, t7793: F, t1882: F, t7790: F, t7808: F, t7795: F, t1566: F, t8232: F) -> (F, F, F, F, F, F, F) {
    let t37322 = t446 * t1866 * t37320;
    let t37324 = t1643 * t1755;
    let t37326 = t446 * t7793 * t37324;
    let t37328 = t1882 * t7790;
    let t37330 = t1882 * t7808;
    let t37332 = t1882 * t7795;
    let t37334 = t8232 * t1566;
    (t37322, t37324, t37326, t37328, t37330, t37332, t37334)
}

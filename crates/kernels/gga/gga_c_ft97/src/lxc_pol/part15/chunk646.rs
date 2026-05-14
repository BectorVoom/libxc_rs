//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 646/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk646<F: Float>(t20224: F, t3187: F, t1909: F, t3194: F, t3193: F, t11902: F, t4607: F, t11906: F, t4612: F, t16034: F, t925: F, t110: F, t1866: F, t20027: F, t4458: F, t447: F, t986: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20225 = t3187 * t20224;
    let t20226 = t1909 * t20225;
    let t20229 = t3194 * t20224;
    let t20230 = t3193 * t20229;
    let t20233 = t11902 * t4607;
    let t20236 = t11906 * t4612;
    let t20239 = t16034 * t925;
    let t20240 = t1909 * t20239;
    let t20244 = t1866 * t110 * t20027;
    let t20248 = t447 * t986 * t4458;
    (t20225, t20226, t20229, t20230, t20233, t20236, t20239, t20240, t20244, t20248)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1347/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1347<F: Float>(t1039: F, t1986: F, t23657: F, t5900: F, t9432: F, t1900: F, t6: F, t91: F, t9252: F, t105909: F, t27158: F, t379: F, t27152: F, t95292: F, t95293: F, t23884: F, t28: F, t586: F, t5890: F) -> (F, F, F, F) {
    let t105919 = t23657 * t9432 * t5900 * t1039 * t1986;
    let t105923 = t91 * t9252 * t6 * t1900;
    let t105926 = t105923 * t105909 * t27158 * t379;
    let t105930 = t95292 * t95293 * t27152 * t379;
    let t105935 = t5890 * t28 * t586 * t23884 * t1039;
    (t105919, t105926, t105930, t105935)
}

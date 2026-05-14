//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1063/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1063<F: Float>(t101882: F, t23054: F, t25901: F, t22914: F, t25570: F, t22878: F, t6414: F, t1286: F, t25552: F, t376: F, t6418: F, t94032: F, t458: F, t6413: F, t5504: F, t11176: F, t1285: F, t25579: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t101883 = 2.0 / 9.0 * t101882;
    let t101898 = t23054 * t25901;
    let t101899 = t101898 / 27.0;
    let t101922 = t22914 * t25570 / 27.0;
    let t101932 = 2.0 / 9.0 * t6414 * t22878;
    let t101943 = t1286 * t376 * t25552 / 9.0;
    let t101949 = t94032 * t6418 / 27.0;
    let t101957 = t6413 * t458;
    let t101959 = t101957 * t5504 / 27.0;
    let t101961 = t1285 * t11176 * t25579;
    (t101883, t101898, t101899, t101922, t101932, t101943, t101949, t101957, t101959, t101961)
}

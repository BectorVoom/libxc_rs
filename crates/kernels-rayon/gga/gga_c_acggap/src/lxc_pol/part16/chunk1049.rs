//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1049/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1049(t36139: f64, t36231: f64, t36236: f64, t36238: f64, t36289: f64, t36327: f64, t36333: f64, t36349: f64, t36370: f64, t36392: f64, t1717: f64, t467: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37879 = 0.32012600194825403606e-1_f64 * t36139;
    let t37918 = 0.90702367218671976884e-1_f64 * t36231;
    let t37922 = 0.45351183609335988442e-1_f64 * t36236;
    let t37923 = 0.19055119163586549766e-2_f64 * t36238;
    let t37940 = 0.37737710747524982482e-2_f64 * t36289;
    let t37957 = 0.18868855373762491241e-1_f64 * t36327;
    let t37961 = 0.12862205435420921092e-1_f64 * t36333;
    let t37970 = 0.45351183609335988442e-1_f64 * t36349;
    let t37982 = 0.34299214494455789578e-2_f64 * t36370;
    let t37994 = 0.34299214494455789578e-2_f64 * t36392;
    let t38519 = t1717 * t467;
    (t37879, t37918, t37922, t37923, t37940, t37957, t37961, t37970, t37982, t37994, t38519)
}

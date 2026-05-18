//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1049/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1049<F: Float>(t36139: F, t36231: F, t36236: F, t36238: F, t36289: F, t36327: F, t36333: F, t36349: F, t36370: F, t36392: F, t1717: F, t467: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t37879 = F::new(0.32012600194825403606e-1) * t36139;
    let t37918 = F::new(0.90702367218671976884e-1) * t36231;
    let t37922 = F::new(0.45351183609335988442e-1) * t36236;
    let t37923 = F::new(0.19055119163586549766e-2) * t36238;
    let t37940 = F::new(0.37737710747524982482e-2) * t36289;
    let t37957 = F::new(0.18868855373762491241e-1) * t36327;
    let t37961 = F::new(0.12862205435420921092e-1) * t36333;
    let t37970 = F::new(0.45351183609335988442e-1) * t36349;
    let t37982 = F::new(0.34299214494455789578e-2) * t36370;
    let t37994 = F::new(0.34299214494455789578e-2) * t36392;
    let t38519 = t1717 * t467;
    (t37879, t37918, t37922, t37923, t37940, t37957, t37961, t37970, t37982, t37994, t38519)
}

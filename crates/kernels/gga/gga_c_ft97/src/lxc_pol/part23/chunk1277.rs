//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1277/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1277<F: Float>(t108278: F, t108284: F, t124054: F, t124058: F, t124061: F, t124065: F, t124069: F, t124074: F, t124079: F, t124083: F, t124087: F, t124112: F, t124114: F, t124121: F, t108291: F, t108334: F, t124093: F, t124096: F, t124101: F, t124106: F, t124110: F, t124118: F, t97030: F) -> (F, F) {
    let t124583 = -t124054 / 9.0 + t124058 / 3.0 - 4.0 / 9.0 * t124061 + 3.0 / 2.0 * t124065 + 3.0 * t124069 + 8.0 / 9.0 * t108278 + 4.0 / 27.0 * t108284 + t124074 / 3.0 + 2.0 * t124079 + 2.0 * t124083 - 6.0 * t124087;
    let t124590 = t124112 / 9.0;
    let t124591 = 2.0 / 27.0 * t124114;
    let t124592 = 2.0 * t124121;
    let t124593 = 16.0 / 9.0 * t108291 - t124093 / 6.0 + t124096 / 9.0 + 2.0 * t124101 - 3.0 / 8.0 * t124106 - 6.0 * t124110 - t124590 - t124591 + t97030 - t124118 + t124592 - t108334;
    (t124583, t124593)
}

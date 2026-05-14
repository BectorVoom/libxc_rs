//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1231/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1231<F: Float>(t26574: F, t6580: F, t5812: F, t62087: F, t5828: F, t77143: F, t22572: F, t23715: F, t30083: F, t4417: F, t554: F, t22632: F, t23774: F, t30058: F, t135: F, t16785: F, t5820: F) -> (F, F, F, F, F, F, F) {
    let t118681 = t6580 * t26574;
    let t118700 = t62087 * t5812;
    let t118703 = t77143 * t5828;
    let t118711 = t23715 * t22572 * t30083;
    let t118714 = t4417 * t554;
    let t118723 = t23774 * t22632 * t30058;
    let t118726 = t16785 * t135 * t5820;
    (t118681, t118700, t118703, t118711, t118714, t118723, t118726)
}

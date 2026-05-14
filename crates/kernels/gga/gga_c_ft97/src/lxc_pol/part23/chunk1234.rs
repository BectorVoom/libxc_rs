//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1234/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1234<F: Float>(t27669: F, t79252: F, t24330: F, t27546: F, t30636: F, t2247: F, t27505: F, t27511: F, t18514: F, t6035: F, t9652: F, t108817: F, t18497: F, t2441: F, t5025: F, t703: F) -> (F, F, F, F, F, F) {
    let t123681 = t79252 * t27669;
    let t123697 = t27546 * t24330 * t30636;
    let t123700 = t27505 * t2247 * t27511;
    let t123709 = t6035 * t9652 * t18514;
    let t123713 = t108817 * t2441 * t18497;
    let t123716 = t703 * t5025;
    (t123681, t123697, t123700, t123709, t123713, t123716)
}

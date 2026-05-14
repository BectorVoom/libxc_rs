//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 859/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk859<F: Float>(t1349: F, t1637: F, t7345: F, t32685: F, t92: F, t33001: F, t376: F, t136304: F, t23701: F, t23823: F, t7203: F, t2001: F, t32772: F, t3392: F, t23711: F, t173: F, t32837: F, t7195: F) -> (F, F, F, F, F, F, F, F, F) {
    let t138705 = 4.0 / 27.0 * t1349 * t1637 * t7345;
    let t138706 = t32685 * t92;
    let t138715 = t1349 * t376 * t33001;
    let t138725 = t23701 * t136304;
    let t138738 = t23823 * t7203;
    let t138739 = t2001 * t138738;
    let t138746 = t3392 * t32772 * t7203;
    let t138761 = t23711 * t136304;
    let t138769 = t7195 * t173 * t32837;
    (t138705, t138706, t138715, t138725, t138738, t138739, t138746, t138761, t138769)
}

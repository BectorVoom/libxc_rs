//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 955/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk955<F: Float>(t23054: F, t32117: F, t2: F, t32325: F, t1317: F, t32121: F, t376: F, t32087: F, t5665: F, t1882: F, t32335: F, t32063: F, t32083: F, t7238: F) -> (F, F, F, F, F, F) {
    let t137110 = t23054 * t32117;
    let t137112 = t2 * t32325;
    let t137124 = t1317 * t376 * t32121;
    let t137131 = t5665 * t376 * t32087;
    let t137163 = t1882 * t32335;
    let t137172 = t7238 * t32063 * t32083;
    (t137110, t137112, t137124, t137131, t137163, t137172)
}

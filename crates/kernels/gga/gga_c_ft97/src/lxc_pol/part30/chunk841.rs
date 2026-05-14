//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 841/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk841<F: Float>(t141203: F, t141363: F, t141367: F, t24211: F, t7437: F, t7440: F, t761: F, t33773: F, t8392: F, t668: F, t7553: F, t33693: F, t33697: F, t33776: F, t33709: F, t33712: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t141607 = 4.0 / 27.0 * t141203;
    let t141651 = 4.0 / 27.0 * t141363;
    let t141652 = 10.0 / 27.0 * t141367;
    let t141671 = 2.0 / 27.0 * t7437 * t24211;
    let t141713 = t761 * t7440;
    let t141722 = t8392 * t33773;
    let t141727 = t7553 * t668;
    let t141744 = t8392 * t33693;
    let t141746 = t8392 * t33697;
    let t141752 = t8392 * t33776;
    let t141759 = t8392 * t33709;
    let t141784 = t8392 * t33712;
    (t141607, t141651, t141652, t141671, t141713, t141722, t141727, t141744, t141746, t141752, t141759, t141784)
}

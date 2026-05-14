//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 788/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk788<F: Float>(t1725: F, t8122: F, t1737: F, t37362: F, t419: F, t420: F, t8112: F, t1675: F, t625: F, t68: F, t72: F, t2247: F, t391: F, t3626: F, t47: F, t14: F, t37678: F) -> (F, F, F, F, F, F, F, F) {
    let t37802 = t1725 * t8122;
    let t37806 = t419 * t420 * t1737 * t37362;
    let t37808 = t1725 * t8112;
    let t37812 = t68 * t1675 * t625 * t72;
    let t37814 = t391 * t2247;
    let t37816 = t68 * t37814 * t72;
    let t37818 = t47 * t3626;
    let t37820 = t68 * t37818 * t72;
    let t37821 = 0.18916624705075445817e-1 * t37820;
    let t37824 = t68 * t37678 * t14 * t72;
    (t37802, t37806, t37808, t37812, t37816, t37820, t37821, t37824)
}

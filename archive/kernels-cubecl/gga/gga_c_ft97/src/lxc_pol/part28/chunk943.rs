//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 943/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk943<F: Float>(t37481: F, t40: F, t1608: F, t22817: F, t52: F, t5522: F, t7837: F, t409: F, t5551: F, t14: F, t32213: F, t1711: F, t64: F) -> (F, F, F, F, F) {
    let t136595 = t40 * t37481;
    let t136597 = t1608 * t22817 * t136595;
    let t136604 = t7837 * t5522 * t52;
    let t136635 = t409 * t5551;
    let t136637 = t32213 * t14;
    let t136642 = t64 * t1711 * t5551;
    (t136597, t136604, t136635, t136637, t136642)
}

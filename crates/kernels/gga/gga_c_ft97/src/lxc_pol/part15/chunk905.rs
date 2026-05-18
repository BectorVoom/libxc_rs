//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 905/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk905<F: Float>(t57491: F, t57527: F, t57620: F, t57718: F, t59170: F, t4545: F, t463: F, t1786: F, t4599: F, t8232: F, t4595: F, t4569: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t59354 = F::new(8.0) / F::new(27.0) * t57491;
    let t59364 = F::new(8.0) / F::new(81.0) * t57527;
    let t59392 = F::new(8.0) / F::new(9.0) * t57620;
    let t59426 = F::new(4.0) / F::new(27.0) * t57718;
    let t59486 = F::new(4.0) / F::new(9.0) * t59170;
    let t59506 = t463 * t4545;
    let t59510 = t1786 * t4545;
    let t59623 = t8232 * t4599;
    let t59629 = t8232 * t4595;
    let t59684 = t8232 * t4569;
    (t59354, t59364, t59392, t59426, t59486, t59506, t59510, t59623, t59629, t59684)
}

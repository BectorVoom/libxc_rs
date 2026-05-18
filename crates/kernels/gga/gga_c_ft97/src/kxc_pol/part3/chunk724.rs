//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 724/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk724<F: Float>(t1882: F, t3856: F, t3974: F, t9735: F, t9701: F, t13746: F, t13753: F, t13780: F, t13794: F, t13809: F, t13811: F, t4354: F, t8675: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t14281 = F::new(2.0) / F::new(27.0) * t1882 * t3856;
    let t14283 = F::new(2.0) / F::new(9.0) * t1882 * t3974;
    let t14317 = F::new(4.0) / F::new(81.0) * t9735;
    let t14318 = F::new(4.0) / F::new(27.0) * t9701;
    let t14327 = F::new(2.0) / F::new(9.0) * t13746;
    let t14329 = t13753 / F::new(9.0);
    let t14336 = t13780 / F::new(27.0);
    let t14341 = F::new(2.0) / F::new(81.0) * t13794;
    let t14346 = t13809 / F::new(27.0);
    let t14347 = F::new(2.0) / F::new(27.0) * t13811;
    let t14421 = F::new(4.0) / F::new(9.0) * t8675 * t4354;
    (t14281, t14283, t14317, t14318, t14327, t14329, t14336, t14341, t14346, t14347, t14421)
}

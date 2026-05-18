//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 904/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk904<F: Float>(t13794: F, t13764: F, t13768: F, t13772: F, t13775: F, t13778: F, t13783: F, t13786: F, t13789: F, t13792: F, t13993: F, t13809: F) -> (F, F) {
    let t13998 = F::new(4.0) / F::new(27.0) * t13794;
    let t13999 = -t13764 / F::new(4.0) + F::new(3.0) / F::new(8.0) * t13768 - t13772 / F::new(2.0) + t13775 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t13778 - t13993 + F::new(2.0) / F::new(3.0) * t13783 - F::new(4.0) / F::new(3.0) * t13786 + F::new(2.0) / F::new(3.0) * t13789 - F::new(8.0) / F::new(3.0) * t13792 + t13998;
    let t14004 = F::new(2.0) / F::new(9.0) * t13809;
    (t13999, t14004)
}

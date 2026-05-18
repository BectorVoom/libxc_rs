//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1155/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1155<F: Float>(t1131: F, t1175: F, t13885: F, t13886: F, t14163: F, t1901: F, t21362: F, t21399: F, t21486: F, t21504: F, t2574: F, t2594: F, t265: F, t42362: F, t446: F, t4965: F, t5053: F, t5073: F, t5181: F, t53923: F, t68662: F, t724: F, t729: F, t81730: F, t81780: F, t88735: F, t89083: F) -> F {
    let t89608 = -F::new(8.0) / F::new(9.0) * t81730 + F::new(8.0) / F::new(3.0) * t446 * t2574 * t265 * t1131 * t21399 - F::new(8.0) / F::new(3.0) * t1901 * t14163 * t89083 - F::new(8.0) / F::new(3.0) * t1901 * t53923 * t21504 - F::new(2.0) * t446 * t729 * t5181 * t5053 - F::new(8.0) / F::new(9.0) * t1901 * t42362 * t4965 * t5073 + F::new(16.0) / F::new(27.0) * t68662 - F::new(8.0) * t1901 * t13885 * t13886 * t21486 - F::new(8.0) / F::new(3.0) * t446 * t724 * t1175 * t21362 - F::new(8.0) / F::new(3.0) * t446 * t2594 * t265 * t88735 + F::new(4.0) / F::new(27.0) * t81780;
    t89608
}

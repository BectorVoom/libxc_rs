//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 582/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk582<F: Float>(t4778: F, t579: F, t91: F, t2124: F, t3318: F, t3335: F, t4654: F, t4658: F, t4662: F, t4666: F, t4671: F, t4717: F, t4755: F) -> (F, F) {
    let t4780 = t91 * t579 * t4778;
    let t4790 = -t4755 / F::new(12.0) + t4780 / F::new(6.0) + t2124 + F::new(2.0) / F::new(27.0) * t3318 + F::new(2.0) / F::new(9.0) * t3335 - F::new(2.0) / F::new(27.0) * t4654 + F::new(2.0) / F::new(9.0) * t4658 + F::new(2.0) / F::new(9.0) * t4662 - t4666 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t4671 - t4717 / F::new(3.0);
    (t4780, t4790)
}

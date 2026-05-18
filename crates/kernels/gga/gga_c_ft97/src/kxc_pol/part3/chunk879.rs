//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 879/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk879<F: Float>(t17667: F, t637: F, t639: F, t12132: F, t12143: F, t12162: F, t12164: F, t12165: F, t12171: F, t12174: F, t12190: F, t12204: F, t12240: F, t17613: F, t17616: F, t17619: F, t17623: F, t17626: F, t17627: F, t17632: F, t17638: F, t17641: F, t2265: F, t631: F, t8718: F) -> F {
    let t17669 = t637 * t639 * t17667;
    let t17673 = -t2265 * t17613 / F::new(3.0) + F::new(2.0) / F::new(27.0) * t2265 * t17616 + F::new(2.0) / F::new(9.0) * t12143 * t17619 - F::new(2.0) / F::new(3.0) * t2265 * t17623 + t17626 + t8718 + t12132 - t17627 / F::new(3.0) + t12162 + t12164 + F::new(10.0) / F::new(27.0) * t12165 - F::new(2.0) / F::new(9.0) * t2265 * t17632 - F::new(4.0) / F::new(9.0) * t12171 + F::new(2.0) * t2265 * t17638 + F::new(4.0) / F::new(3.0) * t2265 * t17641 + t12174 - t12190 + t631 * t17669 / F::new(2.0) + F::new(10.0) / F::new(9.0) * t12204 - t12240;
    t17673
}

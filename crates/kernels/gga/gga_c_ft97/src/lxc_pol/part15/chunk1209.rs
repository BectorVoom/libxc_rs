//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1209/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1209<F: Float>(t72080: F, t72082: F, t83619: F, t83652: F, t83655: F, t83683: F, t89824: F, t89828: F, t89834: F, t89837: F, t89840: F, t89845: F, t89851: F, t89855: F) -> F {
    let t91171 = F::new(4.0) / F::new(9.0) * t83619 - F::new(2.0) / F::new(3.0) * t89824 + F::new(2.0) / F::new(9.0) * t89828 - F::new(4.0) / F::new(3.0) * t83652 + F::new(4.0) / F::new(9.0) * t83655 + t89834 / F::new(3.0) + F::new(4.0) / F::new(3.0) * t89837 - F::new(4.0) / F::new(27.0) * t89840 - F::new(8.0) / F::new(27.0) * t83683 - F::new(40.0) / F::new(243.0) * t89845 + t72080 + t72082 + F::new(4.0) / F::new(3.0) * t89851 + t89855 / F::new(3.0);
    t91171
}

//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1072/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1072<F: Float>(t39674: F, t61462: F, t62134: F, t86608: F, t86937: F, t86942: F, t86946: F, t86950: F, t86954: F, t86958: F, t86962: F, t86966: F, t86970: F, t86975: F, t86979: F) -> F {
    let t87113 = -F::new(15.0) / F::new(16.0) * t86608 + t86937 / F::new(2.0) + F::new(16.0) / F::new(9.0) * t61462 + t39674 - F::new(36.0) * t86942 + F::new(8.0) * t86946 - F::new(80.0) / F::new(81.0) * t86950 - F::new(8.0) * t86954 - t86958 / F::new(3.0) + F::new(8.0) * t86962 + F::new(2.0) * t86966 - F::new(2.0) / F::new(3.0) * t86970 + F::new(16.0) / F::new(3.0) * t62134 - F::new(8.0) * t86975 + F::new(8.0) / F::new(3.0) * t86979;
    t87113
}

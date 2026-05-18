//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1170/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1170<F: Float>(t10238: F, t2649: F, t2745: F, t2892: F, t317: F, t44131: F, t44262: F, t44272: F, t44352: F, t44362: F, t44483: F, t44603: F, t44718: F, t44736: F, t44751: F, t44767: F, t44781: F, t788: F, t829: F, t880: F) -> F {
    let t44789 = -F::new(6.0) * t2745 * t2892 - F::new(8.0) * t10238 * t880 - F::new(12.0) * t44272 - F::new(8.0) * t44483 - F::new(6.0) * t2649 * t2892 - t44718 * t829 * t317 + F::new(48.0) * t44603 - F::new(72.0) * t44262 - t788 * (t44736 + t44751 + t44767 + t44781) * t317 - F::new(2.0) * t44131 - F::new(48.0) * t44352 + F::new(48.0) * t44362;
    t44789
}

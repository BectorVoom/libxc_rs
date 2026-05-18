//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1088/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1088<F: Float>(t41831: F, t41835: F, t41839: F, t41844: F, t41846: F, t41852: F, t41855: F, t41859: F, t41863: F, t41867: F, t41870: F, t41873: F, t41877: F, t41882: F, t41886: F) -> F {
    let t42740 = F::new(8.0) / F::new(3.0) * t41831 - F::new(4.0) * t41835 - F::new(4.0) * t41839 + F::new(6.0) * t41844 + F::new(8.0) * t41846 + F::new(24.0) * t41852 + F::new(16.0) / F::new(3.0) * t41855 + F::new(2.0) * t41859 + F::new(4.0) / F::new(3.0) * t41863 + F::new(8.0) * t41867 - F::new(12.0) * t41870 + F::new(8.0) * t41873 - F::new(4.0) / F::new(3.0) * t41877 - F::new(8.0) / F::new(3.0) * t41882 - F::new(16.0) / F::new(3.0) * t41886;
    t42740
}

//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 757/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk757<F: Float>(t1786: F, t971: F, t1905: F, t463: F, t1912: F, t11878: F, t11882: F, t11883: F, t11887: F, t11897: F, t11899: F, t1901: F, t446: F, t8430: F, t8471: F, t8475: F, t8477: F, t8483: F, t8485: F, t8487: F) -> F {
    let t11902 = t1786 * t971;
    let t11903 = t11902 * t1905;
    let t11906 = t463 * t971;
    let t11907 = t11906 * t1912;
    let t11910 = F::new(2.0) / F::new(9.0) * t446 * t11878 + t11882 - F::new(4.0) / F::new(81.0) * t11883 - t8430 / F::new(9.0) - t446 * t11887 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t8471 - F::new(8.0) / F::new(27.0) * t8475 + t8477 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t8483 - F::new(8.0) / F::new(27.0) * t8485 - F::new(2.0) / F::new(9.0) * t8487 - t11897 - F::new(2.0) / F::new(3.0) * t446 * t11899 + F::new(2.0) / F::new(9.0) * t1901 * t11903 + F::new(2.0) / F::new(9.0) * t1901 * t11907;
    t11910
}

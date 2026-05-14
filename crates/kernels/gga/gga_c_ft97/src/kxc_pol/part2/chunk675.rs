//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 675/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk675<F: Float>(t11906: F, t1912: F, t11878: F, t11882: F, t11883: F, t11887: F, t11897: F, t11899: F, t11903: F, t1901: F, t446: F, t8430: F, t8471: F, t8475: F, t8477: F, t8483: F, t8485: F, t8487: F) -> (F,) {
    let t11907 = t11906 * t1912;
    let t11910 = 2.0 / 9.0 * t446 * t11878 + t11882 - 4.0 / 81.0 * t11883 - t8430 / 9.0 - t446 * t11887 / 3.0 + 2.0 / 9.0 * t8471 - 8.0 / 27.0 * t8475 + t8477 / 9.0 + 2.0 / 9.0 * t8483 - 8.0 / 27.0 * t8485 - 2.0 / 9.0 * t8487 - t11897 - 2.0 / 3.0 * t446 * t11899 + 2.0 / 9.0 * t1901 * t11903 + 2.0 / 9.0 * t1901 * t11907;
    (t11910,)
}

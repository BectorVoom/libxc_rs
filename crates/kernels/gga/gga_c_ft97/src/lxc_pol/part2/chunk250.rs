//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 250/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk250<F: Float>(t301: F, t317: F, t830: F, t876: F, t880: F, t882: F, t332: F, t321: F, t5: F, t170: F, t328: F, t626: F, t327: F, t668: F) -> (F, F, F, F, F) {
    let t885 = -t301 * t880 - t317 * t830 - 2.0 * t876 + 2.0 * t882;
    let t886 = t885 * t332;
    let t889 = t5 * t321;
    let t892 = t170 * t626 * t328 / 6.0;
    let t893 = t327 * t668;
    (t885, t886, t889, t892, t893)
}

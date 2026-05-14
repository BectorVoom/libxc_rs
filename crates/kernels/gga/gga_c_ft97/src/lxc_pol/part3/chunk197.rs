//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 197/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk197<F: Float>(t637: F, t639: F, t643: F, t629: F, t631: F, t634: F, t184: F, t21: F, t231: F, t240: F, t247: F, t342: F, t343: F) -> (F, F, F, F, F, F) {
    let t645 = t637 * t639 * t643;
    let t648 = t629 + t631 * t634 / 6.0 + t631 * t645 / 2.0;
    let t649 = t648 * t184;
    let t650 = t649 * t21;
    let t657 = t231 * t240;
    let t661 = t247 - t342 * t343 * t657 / 4.0;
    (t645, t648, t649, t650, t657, t661)
}

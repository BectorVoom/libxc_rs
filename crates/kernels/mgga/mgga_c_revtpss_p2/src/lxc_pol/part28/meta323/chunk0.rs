//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1333/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1333<F: Float>(t124: F, t836: F, t10779: F, t2749: F, t10777: F, t820: F, t823: F, t844: F, t2751: F, t2681: F, t839: F, t222: F, t9727: F) -> (F, F, F, F, F, F, F) {
    let t10780 = t124 * t836;
    let t10782 = t10779 * t10780 * t2749;
    let t10783 = t10777 * t10782;
    let t10811 = t820 * t823 * t844;
    let t10812 = t10811 * t2751;
    let t10815 = t820 * t823 * t2681;
    let t10816 = t10815 * t839;
    let t10824 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t9727 * t222;
    (t10782, t10783, t10811, t10812, t10815, t10816, t10824)
}

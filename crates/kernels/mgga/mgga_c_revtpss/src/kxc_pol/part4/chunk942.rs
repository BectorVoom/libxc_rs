//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 942/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk942<F: Float>(t10769: F, t245: F, t2482: F, t814: F, t823: F, t136: F, t853: F, t220: F, t124: F, t836: F, t2749: F, t820: F, t844: F, t2751: F, t2681: F, t839: F) -> (F, F, F, F, F, F, F, F) {
    let t10770 = t10769 * t245;
    let t10777 = t2482 * t823 * t814;
    let t10778 = t853 * t136;
    let t10779 = t10778 * t220;
    let t10780 = t124 * t836;
    let t10782 = t10779 * t10780 * t2749;
    let t10783 = t10777 * t10782;
    let t10811 = t820 * t823 * t844;
    let t10812 = t10811 * t2751;
    let t10815 = t820 * t823 * t2681;
    let t10816 = t10815 * t839;
    (t10770, t10777, t10779, t10783, t10811, t10812, t10815, t10816)
}

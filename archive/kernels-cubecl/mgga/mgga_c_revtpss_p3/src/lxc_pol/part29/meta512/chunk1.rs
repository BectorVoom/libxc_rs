//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1834/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1834<F: Float>(t10832: F, t25245: F, t25266: F, t2648: F, t2681: F, t7036: F, t820: F, t839: F, t25260: F, t843: F, t2726: F, t10841: F) -> (F, F, F, F, F, F) {
    let t93043 = t25245 * t10832;
    let t93045 = t25266 * t2648;
    let t93048 = t820 * t7036 * t2681;
    let t93049 = t93048 * t839;
    let t93054 = t820 * t25260 * t843;
    let t93055 = t93054 * t2726;
    let t93058 = t25245 * t10841;
    (t93043, t93045, t93048, t93049, t93055, t93058)
}

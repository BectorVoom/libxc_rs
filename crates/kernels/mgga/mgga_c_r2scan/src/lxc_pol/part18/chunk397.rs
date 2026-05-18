//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 397/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk397<F: Float>(t1743: F, t219: F, t225: F, t234: F, t1398: F, t236: F, t735: F, t424: F, t5: F, t736: F, t378: F, t745: F) -> (F, F, F, F, F, F, F) {
    let t1745 = t219 * t1743 * t225;
    let t1747 = F::new(0.5848223622634646207e0) * t234 * t1745;
    let t1748 = t1398 * t236;
    let t1750 = F::new(0.72290542002011598948e-2) * t735 * t1748;
    let t1751 = t424 * t5;
    let t1752 = t1751 * t736;
    let t1754 = t378 * t745;
    (t1745, t1747, t1748, t1750, t1751, t1752, t1754)
}

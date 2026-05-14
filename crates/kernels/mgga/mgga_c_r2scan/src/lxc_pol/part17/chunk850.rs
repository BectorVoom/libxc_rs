//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 850/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk850<F: Float>(t10744: F, t120: F, t2222: F, t2096: F, t2101: F, t2105: F, t265: F, t254: F, t277: F, t3332: F) -> (F, F, F, F, F) {
    let t10745 = 0.25610080155860322884e0 * t10744;
    let t10748 = t120 * t2222;
    let t10757 = t2101 * t2096 * t265 * t2105;
    let t10758 = t254 * t10757;
    let t10760 = t3332 * t277;
    (t10745, t10748, t10757, t10758, t10760)
}

//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 907/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk907<F: Float>(t11741: F, t3304: F, t545: F, t979: F, t3300: F, t2206: F, t978: F, t146: F, t3305: F, t10781: F, t2553: F, t10856: F, t2842: F, t10894: F, t927: F, t787: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11742 = t3304 * t11741;
    let t11744 = t545 * t979;
    let t11745 = t11744 * t3300;
    let t11747 = t2206 * t978;
    let t11748 = t146 * t11747;
    let t11749 = t11748 * t3305;
    let t11751 = t10781 * t2553;
    let t11753 = t10856 * t2842;
    let t11758 = t10894 * t927;
    let t11760 = t978 * t787;
    (t11742, t11744, t11745, t11747, t11748, t11749, t11751, t11753, t11758, t11760)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 927/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk927<F: Float>(t1937: F, t29432: F, t6993: F, t7586: F, t7316: F, t8764: F, t7239: F, t32101: F, t32102: F, t32104: F, t32107: F, t32109: F, t32112: F, t32116: F, t32825: F, t671: F, t8463: F) -> (F,) {
    let t32843 = t29432 * t1937;
    let t32845 = t7586 * t6993;
    let t32849 = t8764 * t7316;
    let t32850 = t8764 * t7239;
    let t32853 = -2.0 * t32825 * t671 + t32101 - t32102 - 2.0 * t32104 - t32107 - t32109 - t32112 - t32116 - 2.0 * t32843 - 2.0 * t32845 - t32849 + 3.0 * t32850 - t8463;
    (t32853,)
}

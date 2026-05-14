//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1012/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1012<F: Float>(t1998: F, t6081: F, t1856: F, t7614: F, t35259: F, t35261: F, t37464: F, t39797: F, t39802: F, t39807: F, t39809: F, t39811: F, t39813: F, t39815: F, t39817: F, t39819: F, t39822: F, t39825: F, t39829: F, t39831: F) -> (F,) {
    let t39833 = t1998 * t6081;
    let t39835 = t7614 * t1856;
    let t39837 = t35259 + 0.64311027177104605458e-3 * t39797 - t35261 - 0.31448092289604152068e-2 * t39802 - 0.18868855373762491241e-1 * t39807 - 0.17149607247227894789e-2 * t39809 - 0.68598428988911579156e-2 * t39811 + 0.34299214494455789578e-2 * t39813 - 0.51448821741683684367e-2 * t39815 + 0.17149607247227894789e-2 * t39817 + 0.25724410870841842183e-2 * t39819 + t39822 / 24.0 + t39825 / 192.0 + 0.47172138434406228102e-2 * t39829 - 0.17149607247227894789e-2 * t39831 + 0.85748036236139473944e-3 * t39833 + 0.16006300097412701803e-1 * t39835 - t37464;
    (t39837,)
}

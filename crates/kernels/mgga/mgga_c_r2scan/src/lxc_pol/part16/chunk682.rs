//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 682/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk682<F: Float>(t664: F, t673: F, t621: F, t631: F, t5771: F, t225: F, t5270: F, t717: F, t1813: F, t1966: F, t2017: F, t2030: F, t207: F, t5507: F, t5549: F, t5589: F, t5798: F, t5801: F, t5812: F, t5815: F, t5818: F, t5821: F, t5822: F, t5823: F, t674: F, t686: F, t687: F, t690: F, t705: F) -> (F, F, F) {
    let t5829 = t673 * t664;
    let t5832 = t631 * t621;
    let t5834 = 0.12822e1 * t5832 * t5771;
    let t5836 = t717 * t5270 * t225;
    let t5841 = -0.35089341735807877242e1 * t705 * t5798 + 0.57791679765211885293e1 * t5801 * t1813 + 0.96491876992155210402e2 * t687 * t2017 * t1966 + 0.32163958997385070134e2 * t687 * t690 * t5549 + t5812 + t5815 - t5818 + t5821 + 18.0 * t5822 * t5823 - 6.0 * t674 * t2030 * t664 - 0.123288e1 * t5829 * t5507 + t5834 + 0.3903689268108626343e0 * t5836 + 0.123288e1 * t686 * t5589 * t207;
    (t5834, t5836, t5841)
}

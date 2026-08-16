//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1018/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1018<F: Float>(t11986: F, t1592: F, t247: F, t1063: F, t1062: F, t11940: F, t11262: F, t1670: F, t1041: F, t1663: F, t371: F, t676: F) -> (F, F, F, F, F, F) {
    let t15711 = t247 * t11986 * t1592;
    let t15712 = t1063 * t15711;
    let t15716 = t11940 * t1062;
    let t15731 = t11262 * t1670;
    let t15732 = t1041 * t15731;
    let t15749 = t371 * t676 * t1663;
    (t15711, t15712, t15716, t15731, t15732, t15749)
}

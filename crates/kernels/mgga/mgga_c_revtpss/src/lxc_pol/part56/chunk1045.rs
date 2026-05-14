//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1045/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1045<F: Float>(t131200: F, t131216: F, t131226: F, t131331: F, t131356: F, t131362: F, t132107: F, t132116: F, t1921: F, t8970: F, t2172: F, t8240: F, t124440: F, t124442: F, t125172: F, t125174: F, t131183: F, t1464: F, t29469: F, t3: F, t33554: F, t35019: F, t575: F, t5808: F, t8971: F) -> (F, F) {
    let t132119 = t131200 + t131216 + t131226 + t131331 + t131356 + t131362 + t132107 + t132116;
    let t132123 = t8970 * t1921;
    let t132128 = t8240 * t2172;
    let t132132 = t132119 * t3 * t575 + t1464 * t35019 + t1921 * t33554 + 2.0 * t2172 * t29469 + t5808 * t8971 + t124440 + t124442 + 2.0 * t125172 + 2.0 * t125174 + t131183 + t132123 + 2.0 * t132128;
    (t132119, t132132)
}

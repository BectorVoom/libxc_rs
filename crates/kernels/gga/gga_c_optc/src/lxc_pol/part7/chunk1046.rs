//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1046/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1046<F: Float>(t138: F, t6843: F, t2053: F, t658: F, t2086: F, t637: F, t120: F, t6916: F, t1928: F, t121: F, t123: F, t124: F, t1948: F, t2057: F, t2060: F, t2061: F, t2064: F, t21868: F, t22052: F, t22268: F, t22269: F, t22286: F, t22628: F, t22653: F, t22678: F, t22698: F, t22729: F, t3411: F, t641: F, t642: F, t6560: F, t6847: F, t6857: F, t6860: F, t6861: F, t6864: F, t9742: F, t9747: F) -> (F, F) {
    let t22736 = t6843 * t138;
    let t22739 = t2053 * t658;
    let t22744 = t637 * t2086;
    let t22751 = t120 * t6916;
    let t22752 = t1928 * t1928;
    let t22769 = -F::new(0.12897460341341234505e3) * (t22268 + t22269 + t22286 + t22628 + t22653 + t22678 + t22698 + t22729) * t121 * t124 + F::new(0.15476952409609481406e4) * t22736 * t642 - F::new(0.92861714457656888434e4) * t22739 * t2061 + F::new(0.23215428614414222108e4) * t6847 * t2064 + F::new(0.30953904819218962812e5) * t22744 * t6857 - F::new(0.18572342891531377687e5) * t9742 * t6861 + F::new(0.15476952409609481406e4) * t2057 * t6864 - F::new(0.46430857228828444218e5) * t22751 * t124 * t22752 + F::new(0.46430857228828444218e5) * t9747 * t123 * t1928 * t1948 - F::new(0.46430857228828444218e4) * t2060 * t124 * t21868 - F::new(0.61907809638437925624e4) * t3411 * t6860 * t6560 + F::new(0.38692381024023703515e3) * t641 * t124 * t22052;
    (t22752, t22769)
}

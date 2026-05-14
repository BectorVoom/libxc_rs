//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 615/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk615<F: Float>(t10867: F, t13072: F, t10924: F, t787: F, t9824: F, t3427: F, t871: F, t10628: F, t2365: F, t6111: F, t10012: F, t1022: F, t9438: F, t2684: F, t10007: F, t825: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13073 = t10867 * t13072;
    let t13077 = t787 * t10924;
    let t13078 = t13077 * t9824;
    let t13088 = t3427 * t871;
    let t13118 = t2365 * t10628;
    let t13119 = t6111 * t13118;
    let t13141 = t10012 * t1022;
    let t13142 = t9438 * t13141;
    let t13143 = t2684 * t13142;
    let t13149 = t10007 * t1022;
    let t13150 = t9438 * t13149;
    let t13151 = t825 * t13150;
    (t13073, t13077, t13078, t13088, t13118, t13119, t13141, t13142, t13143, t13149, t13150, t13151)
}

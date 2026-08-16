//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1203/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1203<F: Float>(t14711: F, t14754: F, t14784: F, t14811: F, t14841: F, t14878: F, t14889: F, t14936: F, t136: F, t1568: F, t2457: F, t2710: F) -> (F, F) {
    let t14939 = t14711 + t14754 + t14784 + t14811 + t14841 + t14878 + t14889 + t14936;
    let t14946 = t1568 * t136;
    let t14948 = t2710 * t14946 * t2457;
    (t14939, t14948)
}

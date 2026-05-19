//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 452/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk452<F: Float>(t1568: F, t225: F, t257: F, t1559: F, t879: F, t234: F, t213: F, t820: F, t873: F, t878: F) -> (F, F, F) {
    let t1569 = t1568 * t225;
    let t1570 = t1569 * t257;
    let t1573 = t879 * t1559;
    let t1576 = t234 * t1568;
    let t1579 = -t873 + t878 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t1573 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t1576;
    (t1569, t1570, t1579)
}

//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1157/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1157<F: Float>(t2741: F, t6019: F, t5966: F, t775: F, t10698: F, t828: F, t1544: F, t4343: F, t2477: F, t5984: F, t800: F, t5988: F) -> (F, F, F, F, F) {
    let t18491 = t2741 * t6019;
    let t18493 = t5966 * t775;
    let t18495 = t10698 * t828 * t18493;
    let t18498 = t1544 * t4343;
    let t18500 = t2477 * t828 * t18498;
    let t18507 = t800 * t5984 * t775;
    let t18511 = t800 * t5988 * t775;
    (t18491, t18495, t18500, t18507, t18511)
}

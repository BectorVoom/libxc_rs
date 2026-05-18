//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1341/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1341<F: Float>(t20747: F, t247: F, t3719: F, t369: F, t6593: F, t475: F, t467: F, t1260: F, t17307: F, t1256: F, t6602: F, t6595: F) -> (F, F, F, F, F) {
    let t21267 = t247 * t3719 * t20747;
    let t21270 = t6593 * t369;
    let t21271 = t475 * t21270;
    let t21272 = t467 * t21271;
    let t21275 = t17307 * t1260;
    let t21283 = t6602 * t1256;
    let t21285 = t6595 * t1256;
    (t21267, t21272, t21275, t21283, t21285)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2101/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2101<F: Float>(t28056: F, t7732: F, t116: F, t29568: F, t5891: F, t94978: F, t665: F, t94982: F, t1513: F, t4287: F, t25826: F, t25823: F, t5915: F) -> (F, F, F, F, F, F) {
    let t105863 = F::cast_from(4.0_f64) * t7732 * t28056;
    let t105866 = t29568 * t116;
    let t105870 = t94978 * t5891;
    let t105872 = t5891 * t665;
    let t105873 = t94982 * t105872;
    let t105875 = t1513 * t4287;
    let t105876 = t25826 * t105875;
    let t105878 = t25823 * t5915;
    (t105863, t105866, t105870, t105873, t105876, t105878)
}

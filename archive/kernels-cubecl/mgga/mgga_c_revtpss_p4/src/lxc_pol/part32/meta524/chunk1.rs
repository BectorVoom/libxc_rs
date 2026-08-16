//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1829/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1829<F: Float>(t159: F, t8779: F, t218: F, t816: F, t10685: F, t1946: F, t10671: F, t7033: F, t25255: F, t2689: F, t10690: F, t1945: F, t9646: F) -> (F, F, F, F, F, F) {
    let t92993 = t8779 * t159;
    let t92995 = t92993 * t218 * t816;
    let t92997 = t1946 * t10685;
    let t92999 = t7033 * t10671;
    let t93001 = t2689 * t25255;
    let t93007 = t9646 * t1945 * t10690;
    (t92993, t92995, t92997, t92999, t93001, t93007)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1290/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1290<F: Float>(t225: F, t9801: F, t4062: F, t3889: F, t543: F, t1386: F, t2482: F, t814: F, t136: F, t1412: F, t220: F, t124: F, t1398: F) -> (F, F, F, F, F, F, F) {
    let t9802 = t9801 * t225;
    let t9804 = F::cast_from(0.45738002528356795401e-4_f64) * t9802 * t4062;
    let t9810 = t543 * t3889;
    let t9816 = t2482 * t1386 * t814;
    let t9817 = t1412 * t136;
    let t9818 = t9817 * t220;
    let t9819 = t124 * t1398;
    (t9802, t9804, t9810, t9816, t9817, t9818, t9819)
}

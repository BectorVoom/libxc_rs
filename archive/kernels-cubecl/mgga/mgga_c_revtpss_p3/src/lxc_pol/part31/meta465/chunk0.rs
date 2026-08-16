//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1706/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1706<F: Float>(t22046: F, t3936: F, t3938: F, t5659: F, t5673: F, t5674: F, t1399: F, t125: F, t6836: F, t9955: F, t1413: F, t6816: F) -> (F, F, F, F, F) {
    let t22107 = t3936 * t22046 * t3938;
    let t22111 = t5673 * t5674 * t5659;
    let t22115 = t5673 * t22046 * t1399;
    let t22118 = t125 * t6836;
    let t22120 = t9955 * t22118 * t1399;
    let t22125 = t1413 * t6816;
    (t22107, t22111, t22115, t22120, t22125)
}

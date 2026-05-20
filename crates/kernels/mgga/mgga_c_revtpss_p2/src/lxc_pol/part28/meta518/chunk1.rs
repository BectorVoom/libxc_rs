//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1938/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1938<F: Float>(t342: F, t7810: F, t1678: F, t3140: F, t1078: F, t1982: F, t1089: F, t1668: F, t25681: F, t4866: F, t7168: F, t7828: F, t988: F) -> (F, F, F, F, F) {
    let t27616 = t342 * t7810;
    let t27619 = t1678 * t3140;
    let t27621 = t1982 * t27619 * t1078;
    let t27627 = t25681 * t1668 * t1089;
    let t27631 = t7168 * t4866 * t1089;
    let t27634 = t7828 * t988;
    (t27616, t27621, t27627, t27631, t27634)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1868/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1868<F: Float>(t1089: F, t1668: F, t25681: F, t4866: F, t7168: F, t7828: F, t988: F, t7160: F, t1078: F, t11239: F, t1035: F, t1983: F) -> (F, F, F, F, F, F) {
    let t27627 = t25681 * t1668 * t1089;
    let t27631 = t7168 * t4866 * t1089;
    let t27634 = t7828 * t988;
    let t27635 = t7160 * t27634;
    let t27638 = t11239 * t1078;
    let t27639 = t27638 * t1035;
    let t27640 = t1983 * t27639;
    (t27627, t27631, t27635, t27638, t27639, t27640)
}

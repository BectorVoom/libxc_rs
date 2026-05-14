//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1044/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1044<F: Float>(t1580: F, t28447: F, t689: F, t110502: F, t25387: F, t18797: F, t26497: F, t110322: F, t25375: F, t18805: F, t95936: F, t30391: F, t93314: F, t93302: F, t30313: F, t531: F) -> (F, F, F, F, F, F, F, F) {
    let t110591 = t689 * t28447 * t1580;
    let t110600 = t25387 * t110502;
    let t110613 = t26497 * t18797;
    let t110615 = t25375 * t110322;
    let t110639 = t95936 * t18805;
    let t110676 = t30391 * t689;
    let t110677 = t93314 * t110676;
    let t110679 = t93302 * t110676;
    let t111221 = t531 * t30313;
    (t110591, t110600, t110613, t110615, t110639, t110677, t110679, t111221)
}

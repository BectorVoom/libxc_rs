//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2823/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2823<F: Float>(t11408: F, t941: F, t2979: F, t2986: F, t11465: F, t960: F, t2935: F, t2967: F, t11509: F, t3006: F, t2866: F, t2873: F) -> (F, F, F, F, F, F) {
    let t41779 = t941 * t11408;
    let t41785 = t2979 * t2986;
    let t41788 = t960 * t11465;
    let t41799 = t2935 * t2967;
    let t41813 = t3006 * t11509;
    let t41880 = t2866 * t2873;
    (t41779, t41785, t41788, t41799, t41813, t41880)
}

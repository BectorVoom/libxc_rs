//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 863/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk863<F: Float>(t25082: F, t28067: F, t4237: F, t76: F, t13269: F, t38: F, t1497: F, t640: F, t77: F, t4241: F, t84: F, t1470: F, t2242: F) -> (F, F, F, F, F, F) {
    let t28069 = F::cast_from(3.0_f64) * t25082 * t28067;
    let t28089 = t76 * t4237;
    let t28093 = t13269 * t38;
    let t28104 = t640 * t1497;
    let t28105 = t77 * t28104;
    let t28108 = t84 * t4241;
    let t28109 = t77 * t28108;
    let t28112 = t2242 * t1470;
    (t28069, t28089, t28093, t28105, t28109, t28112)
}

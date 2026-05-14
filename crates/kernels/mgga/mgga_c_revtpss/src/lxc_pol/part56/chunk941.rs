//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 941/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk941<F: Float>(t33412: F, t8938: F, t97346: F, t124610: F, t3781: F, t7642: F, t482: F, t494: F, t372: F, t12808: F, t3566: F, t7657: F, t1032: F, t2142: F, t2148: F, t26916: F) -> (F, F, F, F, F, F, F, F, F) {
    let t124635 = t8938 * t97346 * t33412;
    let t124644 = t7642 * t3781 * t124610;
    let t124645 = t482 * t494;
    let t124646 = t372 * t124645;
    let t124650 = t12808 * t124610;
    let t124659 = t3566 * t7657;
    let t124664 = t2142 * t1032;
    let t124665 = t2148 * t124664;
    let t124668 = t7642 * t26916;
    (t124635, t124644, t124645, t124646, t124650, t124659, t124664, t124665, t124668)
}

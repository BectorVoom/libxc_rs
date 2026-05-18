//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1289/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1289<F: Float>(t2142: F, t6622: F, t73: F, t1209: F, t30840: F, t3153: F, t20849: F, t1276: F, t2148: F, t3140: F, t6695: F, t1770: F, t8190: F) -> (F, F, F, F, F, F) {
    let t111844 = t2142 * t6622;
    let t111845 = t111844 * t73;
    let t111865 = t1209 * t30840;
    let t111906 = t111844 * t3153;
    let t112018 = t20849 * t2142;
    let t112048 = t2148 * t6695 * t3140 * t1276;
    let t112075 = t1770 * t8190;
    (t111845, t111865, t111906, t112018, t112048, t112075)
}

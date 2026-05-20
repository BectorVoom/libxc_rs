//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2741/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2741<F: Float>(t14668: F, t14923: F, t124: F, t4423: F, t14686: F, t14931: F, t4366: F, t1544: F, t2645: F, t2722: F, t1558: F, t231: F, t40406: F, t685: F, t72: F, t826: F) -> (F, F, F, F, F, F) {
    let t50409 = t14923 * t14668;
    let t50412 = t124 * t4423;
    let t50415 = t14931 * t14686 * t50412 * t4366;
    let t50418 = t1544 * t2645;
    let t50423 = t1544 * t2722;
    let t50436 = t40406 * t826 * t1558 * t231 * t72 * t685;
    (t50409, t50412, t50415, t50418, t50423, t50436)
}

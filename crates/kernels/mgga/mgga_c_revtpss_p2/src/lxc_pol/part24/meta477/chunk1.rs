//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1463/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1463<F: Float>(t3205: F, t371: F, t6337: F, t676: F, t15731: F, t4879: F, t225: F, t64686: F, t366: F, t19566: F, t3090: F, t1086: F, t19462: F) -> (F, F, F, F, F, F) {
    let t67206 = t3205 * t371 * t676 * t6337;
    let t67473 = t4879 * t15731;
    let t67501 = t64686 * t225;
    let t67502 = t67501 * t366;
    let t67528 = t19566 * t3090;
    let t67551 = t19462 * t1086 * t3090;
    (t67206, t67473, t67501, t67502, t67528, t67551)
}

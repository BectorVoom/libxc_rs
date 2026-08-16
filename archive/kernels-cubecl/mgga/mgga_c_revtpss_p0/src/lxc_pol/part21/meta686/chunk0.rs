//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2503/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2503<F: Float>(t12772: F, t12780: F, t3625: F, t13052: F, t13054: F, t3172: F, t11262: F, t3711: F, t3713: F, t12657: F, t1284: F, t3624: F) -> (F, F, F, F) {
    let t44729 = t3625 * t12772 * t12780;
    let t44748 = t13052 * t3172 * t13054;
    let t44751 = t3711 * t11262 * t3713;
    let t44769 = t12657 * t1284 * t3624;
    (t44729, t44748, t44751, t44769)
}

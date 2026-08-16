//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3167/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3167<F: Float>(t1260: F, t44843: F, t17423: F, t17426: F, t343: F, t56: F, t816: F, t13026: F, t65: F, t12256: F, t12772: F, t17634: F, t3625: F) -> (F, F, F, F, F) {
    let t57520 = t44843 * t1260;
    let t57534 = t17426 * t17423;
    let t57548 = t56 * t343 * t816;
    let t57549 = t65 * t13026;
    let t57550 = t57549 * t12256;
    let t57569 = t3625 * t12772 * t17634;
    (t57520, t57534, t57548, t57550, t57569)
}

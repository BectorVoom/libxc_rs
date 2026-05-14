//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 944/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk944<F: Float>(t6762: F, t6765: F, t6798: F, t6815: F, t6817: F, t6820: F, t6823: F, t8929: F, t8947: F, t8952: F, t8955: F, t8958: F, t8911: F, t6793: F, t8908: F, t788: F) -> (F, F, F, F) {
    let t8960 = -t6815 + 0.5519e0 * t6817 - 0.16557e0 * t6820 - 0.16557e0 * t6823 - t6798 - t8947 + 0.905775e0 * t8929 + 0.80513333333333333334e0 * t6762 - 0.301925e0 * t6765 + 0.19419375e1 * t8952 - 0.412621875e-1 * t8955 + 0.27595e0 * t8958;
    let t8965 = 2.0 / 3.0 * t8911;
    let t8966 = -t6793 + 8.0 / 9.0 * t6762 - t6765 / 3.0 + 4.0 / 9.0 * t8908 - t8965 + t8929;
    let t8967 = t788 * t8966;
    (t8960, t8965, t8966, t8967)
}

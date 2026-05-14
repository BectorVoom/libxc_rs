//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 700/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk700<F: Float>(t225: F, t8085: F, t1903: F, t2097: F, t7296: F, t1882: F, t543: F, t7301: F, t545: F, t2028: F, t1904: F, t2027: F, t2103: F, t213: F, t561: F, t7295: F, t7495: F, t7498: F, t7511: F, t7517: F, t7519: F, t7917: F) -> (F, F, F, F, F, F, F, F) {
    let t8086 = t8085 * t225;
    let t8094 = t2097 * t1903;
    let t8095 = t7296 * t8094;
    let t8099 = t2097 * t1882 * t543;
    let t8100 = t7301 * t8099;
    let t8103 = t545 * t8085;
    let t8104 = t2028 * t8103;
    let t8107 = -t7495 + t7498 + 0.65854491829355115987e0 * t213 * t8086 * t561 - 0.65854491829355115987e0 * t7511 * t1904 + t7517 - t7519 - 0.4336814094102599731e0 * t7917 * t2103 + 0.8673628188205199462e0 * t7295 * t8095 + 0.4336814094102599731e0 * t7295 * t8100 - 0.4336814094102599731e0 * t2027 * t8104;
    (t8086, t8094, t8095, t8099, t8100, t8103, t8104, t8107)
}

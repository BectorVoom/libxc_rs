//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 934/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk934<F: Float>(t33111: F, t687: F, t3721: F, t4905: F, t8601: F, t8616: F, t3179: F, t8598: F, t11706: F, t883: F, t2468: F, t3742: F, t11725: F, t2469: F, t2470: F, t24915: F, t2822: F, t3268: F, t33103: F, t33105: F, t33110: F, t3746: F, t3795: F, t7053: F, t7056: F, t7063: F, t972: F) -> (F, F, F, F, F) {
    let t33113 = 2.0 * t33111 * t687;
    let t33114 = t4905 * t3721;
    let t33116 = 4.0 * t8601 * t8616;
    let t33119 = 4.0 * t8598 * t3179;
    let t33121 = t11706 * t883;
    let t33129 = t3742 * t2468;
    let t33137 = 2.0 * t2469 * t2822 * t3795 - 6.0 * t2822 * t3746 * t7063 + 4.0 * t11725 * t7056 + 2.0 * t2470 * t33129 + 8.0 * t24915 * t3268 - 2.0 * t33121 * t972 - t3795 * t7053 - t33103 + t33105 + t33110 + t33113 + t33114 - t33116;
    (t33113, t33114, t33116, t33119, t33137)
}

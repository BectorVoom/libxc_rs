//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1547;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1548;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta520<F: Float>(t1160: F, t24453: F, t24362: F, t3479: F, t24407: F, t3523: F, t1179: F, t24252: F, t24864: F, t460: F, t5219: F, t6695: F, t1811: F, t20849: F, t6564: F, t1770: F, t12772: F, t24568: F, t5340: F, t24572: F, t5331: F, t11249: F, t24543: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t81791, t81836, t81873, t82050, t82147, t82150) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1547::<F>(t1160, t24453, t24362, t3479, t24407, t3523, t1179, t24252, t24864, t460, t5219, t6695);
        let (t82204, t82217, t82238, t82286, t82289, t82293) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1548::<F>(t1811, t20849, t6564, t1770, t6695, t12772, t24568, t5340, t24572, t5331, t11249, t24543);
    (t81791, t81836, t81873, t82050, t82147, t82150, t82204, t82217, t82238, t82286, t82289, t82293)
}

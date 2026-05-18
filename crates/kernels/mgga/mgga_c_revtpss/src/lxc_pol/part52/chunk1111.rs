//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1111/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1111<F: Float>(t1468: F, t2411: F, t11064: F, t605: F, t30: F, t41154: F, t1568: F, t7063: F, t1113: F, t33: F, t1711: F, t116: F, t28683: F) -> (F, F, F, F, F, F, F, F) {
    let t98658 = t2411 * t1468;
    let t98763 = t11064 * t605;
    let t98785 = t41154 * t30;
    let t98848 = t7063 * t1568;
    let t100974 = t11064 * t1113;
    let t100981 = t41154 * t33;
    let t100987 = t2411 * t1711;
    let t101705 = t116 * t28683;
    (t98658, t98763, t98785, t98848, t100974, t100981, t100987, t101705)
}

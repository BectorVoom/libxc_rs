//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2243/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2243<F: Float>(t11064: F, t1113: F, t27384: F, t27799: F, t98767: F, t33: F, t41154: F, t98786: F, t1711: F, t2411: F, t14365: F, t1544: F, t3351: F) -> (F, F, F, F, F) {
    let t100974 = t11064 * t1113;
    let t100975 = t100974 * t27384;
    let t100978 = t27799 * t98767;
    let t100981 = t41154 * t33;
    let t100982 = t100981 * t98786;
    let t100987 = t2411 * t1711;
    let t100988 = t100987 * t14365;
    let t100993 = t3351 * t1544;
    (t100975, t100978, t100982, t100988, t100993)
}

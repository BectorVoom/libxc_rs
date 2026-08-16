//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1125/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1125<F: Float>(t441: F, t4533: F, t2169: F, t1295: F, t1657: F, t8121: F, t915: F, t233: F, t2209: F, t4534: F, t235: F, t5398: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27746 = t4533 * t441;
    let t27747 = t2169 * t27746;
    let t27749 = t1657 * t1295;
    let t27750 = t2169 * t27749;
    let t27752 = t915 * t8121;
    let t27753 = t233 * t27752;
    let t27755 = t4534 * t2209;
    let t27756 = t233 * t27755;
    let t27758 = t235 * t5398;
    (t27746, t27747, t27749, t27750, t27752, t27753, t27755, t27756, t27758)
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1289/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1289<F: Float>(t1009: F, t6533: F, t1704: F, t829: F, t100264: F, t101084: F, t101101: F, t27812: F, t2894: F, t4580: F, t71184: F, t7703: F, t7704: F, t922: F, t92730: F, t93592: F, t95775: F, t95779: F, t95783: F, t95798: F, t96108: F, t9924: F) -> (F, F, F) {
    let t101111 = t1009 * t6533;
    let t101122 = t1704 * t829;
    let t101127 = F::new(0.37134344353515625001e-4) * t27812 * t101101 - F::new(0.88437037037037037035e-2) * t100264 - F::new(0.92673611111111111112e-3) * t95775 - F::new(0.36848765432098765431e-3) * t92730 - t95779 + t95783 - F::new(0.46336805555555555557e-3) * t7703 * t9924 * t101111 * t922 + F::new(0.46336805555555555556e-3) * t7703 * t2894 * t7704 * t71184 + t95798 - F::new(0.13901041666666666667e-2) * t7703 * t101084 - F::new(0.92673611111111111112e-3) * t93592 * t96108 * t4580 * t101122;
    (t101111, t101122, t101127)
}

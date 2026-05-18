//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1185/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1185<F: Float>(t95585: F, t27769: F, t2861: F, t27815: F, t7703: F, t9938: F, t14443: F, t27821: F, t14570: F, t283: F, t990: F, t9588: F) -> (F, F, F, F, F, F, F) {
    let t95586 = F::new(0.66327777777777777776e-2) * t95585;
    let t95587 = t2861 * t27769;
    let t95605 = F::new(0.15445601851851851852e-3) * t7703 * t9938 * t27815;
    let t95606 = t14443 * t27821;
    let t95608 = F::new(0.15445601851851851852e-3) * t7703 * t95606;
    let t95640 = t14570 * t283 * t990;
    let t95655 = t9588 * t283;
    (t95586, t95587, t95605, t95606, t95608, t95640, t95655)
}

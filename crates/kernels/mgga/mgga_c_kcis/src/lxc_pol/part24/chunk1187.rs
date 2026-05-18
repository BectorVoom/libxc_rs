//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1187/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1187<F: Float>(t7703: F, t95684: F, t14443: F, t27957: F, t27953: F, t9938: F, t27789: F, t2861: F, t27793: F, t27842: F, t2822: F, t13398: F, t27846: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t95779 = F::new(0.46336805555555555556e-3) * t7703 * t95684;
    let t95781 = t14443 * t27957;
    let t95783 = F::new(0.15445601851851851852e-3) * t7703 * t95781;
    let t95798 = F::new(0.15445601851851851852e-3) * t7703 * t9938 * t27953;
    let t95815 = t2861 * t27789;
    let t95816 = F::new(0.22109259259259259258e-2) * t95815;
    let t95817 = t2861 * t27793;
    let t95826 = t2822 * t27842;
    let t95827 = F::new(0.22109259259259259258e-2) * t95826;
    let t95828 = t13398 * t27846;
    (t95779, t95781, t95783, t95798, t95815, t95816, t95817, t95826, t95827, t95828)
}

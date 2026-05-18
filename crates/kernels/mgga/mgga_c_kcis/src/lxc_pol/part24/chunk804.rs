//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 804/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk804<F: Float>(t291: F, t4951: F, t1014: F, t4925: F, t4768: F, t978: F, t2861: F, t4986: F, t4793: F, t9429: F, t4815: F, t1017: F, t342: F, t86: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14554 = t4951 * t291;
    let t14567 = t1014 * t4925;
    let t14568 = F::new(0.33163888888888888888e-2) * t14567;
    let t14570 = t4768 * t978;
    let t14576 = t2861 * t4986;
    let t14577 = F::new(0.22109259259259259258e-2) * t14576;
    let t14607 = t9429 * t4793;
    let t14609 = t2861 * t4815;
    let t14627 = t86 * t1017 * t342;
    (t14554, t14567, t14568, t14570, t14576, t14577, t14607, t14609, t14627)
}

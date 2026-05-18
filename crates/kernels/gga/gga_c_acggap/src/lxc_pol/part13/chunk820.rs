//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 820/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk820<F: Float>(t1327: F, t142: F, t8888: F, t599: F, t8406: F, t1181: F, t7346: F, t7678: F, t7686: F, t7697: F, t7698: F, t7710: F, t7713: F, t7718: F, t7721: F, t7726: F, t8876: F, t8879: F, t8882: F, t8885: F) -> (F, F, F, F) {
    let t8889 = t142 * t1327;
    let t8890 = t8888 * t8889;
    let t8896 = t599 * t8406;
    let t8897 = t1181 * t8896;
    let t8898 = t7346 * t8897;
    let t8900 = -t8876 / F::new(64.0) - t8879 / F::new(192.0) + t7678 + F::new(0.20007875121765877254e-2) * t7686 - t7697 - F::new(0.28015625e-1) * t8882 + t8885 / F::new(48.0) + t8890 / F::new(48.0) - F::new(0.21437009059034868486e-3) * t7698 + F::new(0.31448092289604152067e-3) * t7710 - F::new(0.42874018118069736972e-3) * t7713 - t7718 - F::new(0.10718504529517434243e-3) * t7721 - t7726 + F::new(0.10718504529517434243e-3) * t8898;
    (t8889, t8896, t8897, t8900)
}

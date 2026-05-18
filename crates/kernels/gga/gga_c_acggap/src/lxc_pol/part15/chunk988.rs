//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 988/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk988<F: Float>(t1530: F, t31056: F, t3073: F, t33953: F, t4241: F, t13364: F, t13299: F, t30769: F, t4349: F, t7741: F, t30773: F, t30775: F) -> (F, F, F, F, F, F, F) {
    let t34823 = t1530 * t31056;
    let t34833 = t3073 * t31056;
    let t34834 = t33953 * t4241;
    let t34836 = t34833 * t13364 * t34834;
    let t34839 = t34833 * t13299 * t34834;
    let t34843 = F::new(0.68598428988911579156e-2) * t30769;
    let t34844 = t7741 * t4349;
    let t34846 = F::new(0.42874018118069736972e-3) * t30773;
    let t34847 = F::new(0.17149607247227894789e-2) * t30775;
    (t34823, t34836, t34839, t34843, t34844, t34846, t34847)
}

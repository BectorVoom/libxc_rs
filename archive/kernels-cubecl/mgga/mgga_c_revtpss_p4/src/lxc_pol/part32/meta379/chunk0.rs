//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1338/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1338<F: Float>(t1729: F, t2439: F, t5098: F, t698: F, t16708: F, t16710: F, t16712: F, t5095: F, t3523: F, t5180: F, t1737: F, t3451: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16876 = t2439 * t1729;
    let t16892 = t698 * t5098;
    let t16893 = F::cast_from(0.21908444444444444444e0_f64) * t16892;
    let t16915 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t16708;
    let t16916 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t16710;
    let t16917 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t16712;
    let t16929 = F::cast_from(0.39862222222222222222e0_f64) * t16710;
    let t16931 = t698 * t5095;
    let t16988 = t5180 * t3523;
    let t17010 = F::cast_from(0.2283111111111111111e-1_f64) * t16710;
    let t17011 = F::cast_from(0.11415555555555555555e-1_f64) * t16712;
    let t17023 = t1737 * t3451;
    (t16876, t16892, t16893, t16915, t16916, t16917, t16929, t16931, t16988, t17010, t17011, t17023)
}

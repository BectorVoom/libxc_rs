//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1017/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1017<F: Float>(t35500: F, t7380: F, t34050: F, t2095: F, t33901: F, t33884: F, t1998: F, t4503: F, t5124: F, t7647: F, t7310: F, t8878: F) -> (F, F, F, F, F, F, F) {
    let t35909 = t7380 * t35500;
    let t35910 = F::new(0.4584375e-1) * t35909;
    let t35911 = t7380 * t34050;
    let t35912 = F::new(0.4584375e-1) * t35911;
    let t35913 = t2095 * t33901;
    let t35914 = F::new(0.305625e-1) * t35913;
    let t35915 = t2095 * t33884;
    let t35916 = F::new(0.305625e-1) * t35915;
    let t35917 = t1998 * t4503;
    let t35918 = F::cast_from(0.17149607247227894789e-2_f64) * t35917;
    let t35919 = t7647 * t5124;
    let t35920 = F::cast_from(0.17149607247227894789e-2_f64) * t35919;
    let t35924 = t7310 * t8878;
    (t35910, t35912, t35914, t35916, t35918, t35920, t35924)
}

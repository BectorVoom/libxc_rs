//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 668/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk668(t2303: f64, t2308: f64, t3017: f64, t3059: f64, t3732: f64, t3744: f64, t3748: f64, t3752: f64, t3754: f64, t3759: f64, t3763: f64) -> f64 {
    let t3819 = -0.1294625e1_f64 * t3744 + 0.258925e1_f64 * t3748 + t2303 - 0.60385e0_f64 * t3017 + 0.905775e0_f64 * t3732 + 0.82524375e-1_f64 * t3752 + 0.16504875e0_f64 * t3754 + t2308 - 0.33114e0_f64 * t3059 + 0.248355e0_f64 * t3759 + 0.248355e0_f64 * t3763;
    t3819
}

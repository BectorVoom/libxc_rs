//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1299/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1299(t1020: f64, t7719: f64, t95893: f64, t14563: f64, t5329: f64, t7691: f64, t27876: f64, t2822: f64, t4792: f64, t92701: f64, t13186: f64, t26760: f64) -> (f64, f64, f64, f64, f64) {
    let t95895 = t1020 * t95893 * t7719;
    let t95898 = t5329 * t7691 * t14563;
    let t95903 = t2822 * t27876;
    let t95906 = t1020 * t92701 * t4792;
    let t95909 = t1020 * t26760 * t13186;
    (t95895, t95898, t95903, t95906, t95909)
}

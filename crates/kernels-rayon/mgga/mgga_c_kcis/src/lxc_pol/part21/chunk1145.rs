//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1145/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1145(t3489: f64, t4981: f64, t7699: f64, t8030: f64, t1014: f64, t8054: f64, t5019: f64, t7726: f64, t303: f64, t15573: f64, t8041: f64, t2173: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27964 = t4981 * t3489;
    let t27967 = t8030 * t7699;
    let t27969 = t1014 * t8054;
    let t27971 = t7726 * t5019;
    let t27972 = t303 * t27971;
    let t27974 = t15573 * t8041;
    let t27975 = t2173 * t27974;
    (t27964, t27967, t27969, t27971, t27972, t27974, t27975)
}

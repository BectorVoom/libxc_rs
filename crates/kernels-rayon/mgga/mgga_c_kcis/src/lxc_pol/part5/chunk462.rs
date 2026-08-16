//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 462/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk462(t1282: f64, t1779: f64, t1821: f64, t1864: f64, t187: f64, t1872: f64, t437: f64, t236: f64, t487: f64, sigma2: f64) -> (f64, f64, f64) {
    let t1876 = t1779 - t1821 + t187 * (-t1282 * t1872 + t1864 * t437 - t1779 + t1821);
    let t1877 = t236 * t1876;
    let t1880 = 1.0_f64 / t487;
    let t1881 = sigma2 * t1880;
    (t1876, t1877, t1881)
}

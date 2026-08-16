//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 844/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk844(t174: f64, t236: f64, t6883: f64, t233: f64, t1926: f64, t638: f64, t1881: f64, t1886: f64, t2133: f64, t6284: f64, t447: f64, t637: f64, t446: f64, sigma2: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t175 = t174 <= zeta_threshold;
    let t6884 = t236 * t6883;
    let t6885 = t233 * t6884;
    let t6886 = t6885 / 16.0_f64;
    let t6887 = 1.0_f64 / t1926;
    let t6888 = sigma2 * t6887;
    let t6889 = t6888 * t638;
    let t6890 = t6889 / 8.0_f64;
    let t6891 = t1881 * t1886;
    let t6892 = t6891 / 8.0_f64;
    let t6893 = t1881 * t2133;
    let t6894 = t6893 / 8.0_f64;
    let t6895 = piecewise3(t175, 0.0_f64, t6284);
    let t6896 = t447 * t6895;
    let t6897 = t6896 * t637;
    let t6898 = t446 * t6897;
    (t6884, t6886, t6888, t6890, t6892, t6894, t6896, t6898)
}

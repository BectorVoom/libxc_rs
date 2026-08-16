//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 780/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk780(t1877: f64, t2057: f64, t2249: f64, t22951: f64, t22961: f64, t22964: f64, t22968: f64, t23296: f64, t23299: f64, t23302: f64, t24191: f64, t24335: f64, t24339: f64, t24344: f64, t25: f64, t2522: f64, t4314: f64, t606: f64, t6542: f64, t6671: f64, t7110: f64, t7114: f64) -> f64 {
    let t24355 = 3.0_f64 * t4314 * t2057 * t22951 + 3.0_f64 * t2522 * t7110 * t6542 - 3.0_f64 * t24191 * t22961 + 3.0_f64 * t2522 * t2057 * t22964 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t22968 + t1877 * t24335 * t25 / 2.0_f64 - t1877 * t24339 * t6671 + t1877 * t7110 * t606 + t1877 * t24344 * t23296 - t1877 * t7114 * t23299 - t1877 * t7114 * t23302 / 2.0_f64 + t1877 * t2057 * t2249 / 2.0_f64;
    t24355
}

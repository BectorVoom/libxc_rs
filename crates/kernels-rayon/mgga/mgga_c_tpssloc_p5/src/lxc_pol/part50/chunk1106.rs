//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1106/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1106(t1052: f64, t1920: f64, t1956: f64, t23327: f64, t25755: f64, t25778: f64, t30798: f64, t32909: f64, t32913: f64, t32917: f64, t32924: f64, t32965: f64, t32970: f64, t32973: f64, t32976: f64, t32981: f64, t4557: f64, t6687: f64, t8407: f64) -> f64 {
    let t32984 = -6.0_f64 * t1052 * t32909 + 2.0_f64 * t1052 * t32913 + 4.0_f64 * t1052 * t32917 - 2.0_f64 * t25755 * t1956 + 0.16449340668482264365e-1_f64 * t1920 * t32924 - t1052 * t32965 - 2.0_f64 * t25778 * t1956 - 0.54831135561607547883e-2_f64 * t23327 * t32970 + 0.16449340668482264365e-1_f64 * t6687 * t32973 - 0.16449340668482264365e-1_f64 * t6687 * t32976 - t4557 * t8407 + t30798 + 0.3289868133696452873e-1_f64 * t6687 * t32981;
    t32984
}

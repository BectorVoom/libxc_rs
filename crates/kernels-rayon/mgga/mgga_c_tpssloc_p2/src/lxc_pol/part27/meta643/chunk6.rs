//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2195/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2195(t25373: f64, t57921: f64, t1530: f64, t2249: f64, t16596: f64, t81547: f64, t1877: f64, t1915: f64, t22951: f64, t22959: f64, t22968: f64, t23295: f64, t23296: f64, t23302: f64, t25013: f64, t2522: f64, t25354: f64, t25358: f64, t4314: f64, t606: f64, t6542: f64, t6670: f64, t7541: f64, t87953: f64, t87957: f64, t87961: f64, t87975: f64, t87978: f64, t87981: f64, t87984: f64) -> f64 {
    let t87988 = t25373 * t57921;
    let t87994 = t2249 * t1530;
    let t87998 = t81547 * t16596;
    let t88001 = 3.0_f64 / 2.0_f64 * t2522 * t1915 * t87953 + 3.0_f64 * t2522 * t1915 * t87957 + t1877 * t23295 * t87961 + t1877 * t25354 * t606 + 3.0_f64 * t2522 * t25354 * t6542 - t1877 * t25358 * t23302 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t7541 * t22968 + t1877 * t87975 * t23296 + 6.0_f64 * t25013 * t87978 + 3.0_f64 * t25013 * t87981 - t1877 * t6670 * t87984 / 2.0_f64 + 3.0_f64 * t22959 * t87988 + 3.0_f64 * t4314 * t7541 * t22951 - t1877 * t6670 * t87994 / 2.0_f64 - 3.0_f64 * t22959 * t87998;
    t88001
}

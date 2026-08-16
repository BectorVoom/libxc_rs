//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1007/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1007(t1409: f64, t2132: f64, t2136: f64, t460: f64, t4928: f64, t7320: f64, t210: f64, t7998: f64, t1193: f64, t8020: f64, t1198: f64, t2134: f64, t24723: f64, t24729: f64, t24733: f64, t24741: f64, t4950: f64, t4954: f64, t4980: f64, t4984: f64, t5046: f64, t7310: f64, t7316: f64, t7321: f64, t8028: f64, t8031: f64, t8035: f64) -> (f64, f64, f64) {
    let t27650 = t2132 * t1409;
    let t27651 = t27650 * t2136;
    let t27654 = t4928 * t460;
    let t27655 = t27654 * t7320;
    let t27674 = t7998 * t210;
    let t27677 = t8020 * t1193;
    let t27679 = -0.10093189023535097714e-3_f64 * t27651 + 0.10093189023535097714e-3_f64 * t24723 - 0.10093189023535097714e-3_f64 * t2134 * t27655 + 0.10093189023535097714e-3_f64 * t7316 * t8035 - t24741 * t4950 / 2304.0_f64 - t24741 * t4954 / 2304.0_f64 + t24729 * t4980 / 768.0_f64 - t24733 * t4984 / 1536.0_f64 - t7310 * t5046 / 288.0_f64 + 0.80745512188280781712e-3_f64 * t8028 * t7321 + 0.10093189023535097714e-3_f64 * t8031 * t7321 + t27674 * t1198 / 108.0_f64 - t27677 / 108.0_f64;
    (t27650, t27654, t27679)
}

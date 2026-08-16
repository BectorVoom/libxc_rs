//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1024/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1024(t1266: f64, t2165: f64, t2167: f64, t2314: f64, t26006: f64, t26141: f64, t26144: f64, t26145: f64, t26147: f64, t26150: f64, t26153: f64, t26157: f64, t4026: f64, t4028: f64, t4034: f64, t5361: f64, t7271: f64, t7983: f64, t7989: f64) -> f64 {
    let t27878 = -t1266 * t7983 - t2165 * t4026 + t2167 * t5361 - 2.0_f64 * t2314 * t7989 - 2.0_f64 * t4028 * t7271 - 2.0_f64 * t4034 * t7989 - t26006 - t26141 - t26144 - t26145 + t26147 - t26150 + t26153 + t26157;
    t27878
}

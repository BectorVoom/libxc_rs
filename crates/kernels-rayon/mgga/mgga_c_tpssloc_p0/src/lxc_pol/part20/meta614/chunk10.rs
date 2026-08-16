//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2214/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2214(t12620: f64, t12630: f64, t1427: f64, t1434: f64, t2244: f64, t2245: f64, t2284: f64, t2304: f64, t33: f64, t3997: f64, t3998: f64, t4018: f64, t45892: f64, t45931: f64, t45977: f64, t629: f64, t642: f64, t66: f64, t72: f64, t80: f64, t9251: f64, t9313: f64, t9339: f64) -> f64 {
    let t45986 = t2284 * t4018 / 8.0_f64 + t629 * t12620 / 8.0_f64 + t66 * t72 * t45892 / 24.0_f64 + t9313 * t1434 / 24.0_f64 - t9251 * t1434 / 4.0_f64 - t2245 * t4018 / 4.0_f64 - t2244 * t3997 * t80 / 4.0_f64 - t12630 * t642 / 4.0_f64 + t33 * (t45931 + t45977) * t80 / 24.0_f64 + t3998 * t2304 / 8.0_f64 + t1427 * t9339 / 24.0_f64;
    t45986
}

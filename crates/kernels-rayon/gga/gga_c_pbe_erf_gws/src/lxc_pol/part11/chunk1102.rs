//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1102/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1102(t47371: f64, t1044: f64, t10691: f64, t12596: f64, t12814: f64, t12869: f64, t1620: f64, t1621: f64, t186: f64, t198: f64, t25208: f64, t2607: f64, t2615: f64, t30889: f64, t31133: f64, t31267: f64, t3390: f64, t3410: f64, t3414: f64, t3456: f64, t3473: f64, t3488: f64, t3553: f64, t40855: f64, t4927: f64, t5218: f64, t561: f64, t639: f64) -> (f64, f64) {
    let t47638 = -12.0_f64 * t47371;
    let t47672 = 4.0_f64 / 15.0_f64 * t561 * t186 * t198 * t47638 - 8.0_f64 / 45.0_f64 * t30889 + 8.0_f64 / 5.0_f64 * t3488 * t3456 - 8.0_f64 / 5.0_f64 * t1620 * t1621 * t10691 * t3553 - 16.0_f64 / 15.0_f64 * t1620 * t1621 * t2607 * t12869 + 16.0_f64 / 15.0_f64 * t639 * t1621 * t40855 * t1044 + 32.0_f64 / 15.0_f64 * t2615 * t12814 + 16.0_f64 / 15.0_f64 * t639 * t4927 * t3473 * t3390 - 64.0_f64 / 15.0_f64 * t25208 * t12596 - 32.0_f64 / 15.0_f64 * t5218 * t31267 * t3414 - 32.0_f64 / 15.0_f64 * t5218 * t31133 * t3410;
    (t47638, t47672)
}

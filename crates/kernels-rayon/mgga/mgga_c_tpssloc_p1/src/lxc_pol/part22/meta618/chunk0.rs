//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2149/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2149(t3610: f64, t52627: f64, t1227: f64, t1653: f64, t248: f64, t45293: f64, t15730: f64, t3536: f64, t3577: f64, t44951: f64, t4953: f64, t11677: f64, t15245: f64) -> (f64, f64, f64, f64, f64) {
    let t52628 = t3610 * t52627;
    let t52680 = t1227 * t248 * t45293 * t1653;
    let t52731 = t3536 * t15730;
    let t52732 = t52731 / 4608.0_f64;
    let t52758 = t3577 * t44951 * t4953;
    let t52759 = t52758 / 6912.0_f64;
    let t52766 = t15245 * t11677;
    (t52628, t52680, t52732, t52759, t52766)
}

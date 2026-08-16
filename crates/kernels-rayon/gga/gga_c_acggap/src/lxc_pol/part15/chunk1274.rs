//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1274/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1274(t1734: f64, t694: f64, t8034: f64, t9826: f64, t10039: f64, t104: f64, t2407: f64, t3952: f64, t105: f64, t1954: f64, t2170: f64, t2249: f64, t36715: f64, t36729: f64, t36744: f64, t38519: f64, t38573: f64, t41001: f64, t41006: f64, t41042: f64, t41065: f64, t41086: f64, t41111: f64, t41145: f64, t41169: f64, t41194: f64, t41225: f64, t41246: f64, t41267: f64, t41295: f64, t42189: f64, t42205: f64, t42225: f64, t42258: f64, t42284: f64, t469: f64, t567: f64, t6596: f64, t8382: f64, t9096: f64, t9098: f64, t9121: f64, t9469: f64) -> f64 {
    let t42293 = t694 * t8034 * t1734;
    let t42298 = t694 * t9826;
    let t42300 = t104 * t10039;
    let t42307 = t2407 * t3952;
    let t42311 = 6.0_f64 * t567 * t41001 * t9469 - t36715 + 2.0_f64 * t41006 + 2.0_f64 * t567 * t2249 * t6596 + 3.0_f64 * t567 * t2170 * t38573 + t567 * t105 * (t41042 + t41065 + t41086 + t41111 + t41145 + t41169 + t41194 + t41225 + t41246 + t41267 + t41295 + t42189 + t42205 + t42225 + t42258 + t42284) * t469 + 3.0_f64 * t42293 - 6.0_f64 * t9096 * t36729 * t38519 - 6.0_f64 * t42298 + 3.0_f64 * t567 * t42300 * t1954 + 6.0_f64 * t567 * t9121 * t8382 + 4.0_f64 * t9096 * t42307 * t9098 - t36744;
    t42311
}

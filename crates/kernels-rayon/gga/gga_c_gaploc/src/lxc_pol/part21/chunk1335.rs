//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1335/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1335(t1411: f64, t3395: f64, t587: f64, t2365: f64, t2366: f64, t4379: f64, t7892: f64, t10241: f64, t9448: f64, t15482: f64, t20560: f64, t9439: f64) -> (f64, f64, f64, f64) {
    let t34796 = t587 * t1411 * t3395;
    let t34797 = 0.59644551483876721719e0_f64 * t34796;
    let t34800 = t4379 * t2365 * t2366 * t7892;
    let t34801 = 0.89376224879626066674e-1_f64 * t34800;
    let t34814 = t9448 * t10241;
    let t34817 = 0.5680433474654925878e0_f64 * t20560 * t15482 * t34814;
    let t34818 = t9439 * t10241;
    (t34797, t34801, t34817, t34818)
}

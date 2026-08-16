//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1380/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1380(t1: f64, t38285: f64, t544: f64, t1424: f64, t30246: f64, t30250: f64, t30253: f64, t30260: f64, t30263: f64, t30265: f64, t30288: f64, t30294: f64, t34249: f64, t34251: f64, t34253: f64, t34256: f64, t34258: f64, t34260: f64, t34261: f64) -> f64 {
    let t38486 = t544 * t38285 * t1;
    let t38489 = -t34249 + t34251 + t34253 + t34256 + t34258 - 0.10224780254378866581e1_f64 * t30246 - 0.76685851907841499354e0_f64 * t30250 + t30253 - 0.38342925953920749677e0_f64 * t30260 - t34260 + t30263 - t30265 - t30288 + t30294 - 0.79445533226334281486e-1_f64 * t38486 * t1424 + t34261;
    t38489
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1044/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1044(t15478: f64, t585: f64, t4324: f64, t9439: f64, t1428: f64, t4461: f64, t103: f64, t23: f64, t417: f64, t8: f64, t1210: f64, t62: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18821 = t585 * t15478;
    let t18823 = t9439 * t4324;
    let t18970 = t4461 * t1428;
    let t19077 = t23 * t103;
    let t19223 = t8 * t417;
    let t19244 = t62 * t1210;
    (t18821, t18823, t18970, t19077, t19223, t19244)
}

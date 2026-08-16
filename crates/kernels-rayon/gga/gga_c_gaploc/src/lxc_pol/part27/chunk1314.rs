//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1314/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1314(t204: f64, t34378: f64, t587: f64, t10421: f64, t21417: f64, t10311: f64, t4379: f64, t30404: f64, t10314: f64, t20800: f64, t6963: f64, t18535: f64, t19: f64, t584: f64, t60: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34381 = 0.18404604457881959845e2_f64 * t587 * t204 * t34378;
    let t34382 = t10421 * t21417;
    let t34383 = 0.59584149919750711116e-1_f64 * t34382;
    let t34385 = t4379 * t10311;
    let t34386 = 0.29792074959875355558e-1_f64 * t34385;
    let t34394 = 0.15976219147466979032e-1_f64 * t30404;
    let t34397 = 0.95334639871601137784e0_f64 * t6963 * t20800 * t10314;
    let t34400 = t584 * t18535 * t19 * t60;
    (t34381, t34383, t34386, t34394, t34397, t34400)
}

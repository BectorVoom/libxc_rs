//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1412/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1412(t10470: f64, t4849: f64, t10430: f64, t587: f64, t589: f64, t10438: f64, t1391: f64, t31160: f64, t31163: f64, t31166: f64, t31169: f64, t31172: f64, t31175: f64, t31178: f64, t34928: f64, t34931: f64, t34935: f64, t34937: f64, t34939: f64) -> f64 {
    let t34941 = 0.51123901271894332902e1_f64 * t4849 * t10470;
    let t34943 = t587 * t589 * t10430;
    let t34944 = 0.51123901271894332902e0_f64 * t34943;
    let t34946 = t587 * t1391 * t10438;
    let t34947 = 0.2698205900461089792e0_f64 * t34946;
    let t34948 = t31160 - t31163 - t31166 - t31169 - t31172 - t31175 + t31178 - t34928 + t34931 + t34935 - t34937 + t34939 - t34941 + t34944 - t34947;
    t34948
}

//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1406/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1406(t35231: f64, t35234: f64, t35240: f64, t35243: f64, t35246: f64, t35249: f64, t35252: f64, t35254: f64, t35257: f64, t35259: f64, t35263: f64, t35269: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37244 = 0.11372686522837130914e-5_f64 * t35231;
    let t37245 = 0.10567613244746075633e-6_f64 * t35234;
    let t37249 = 0.40483072916666666669e-4_f64 * t35240;
    let t37250 = 0.2698871527777777778e-4_f64 * t35243;
    let t37251 = 0.2698871527777777778e-4_f64 * t35246;
    let t37252 = 0.17149079499421296297e-4_f64 * t35249;
    let t37253 = 0.2748593934505475288e-6_f64 * t35252;
    let t37254 = 0.36652500116630512966e-6_f64 * t35254;
    let t37255 = 0.41030519691600762993e-3_f64 * t35257;
    let t37256 = 0.94685814672924837674e-4_f64 * t35259;
    let t37257 = 0.1500544456199363426e-4_f64 * t35263;
    let t37260 = 0.84412963981222021456e-7_f64 * t35269;
    (t37244, t37245, t37249, t37250, t37251, t37252, t37253, t37254, t37255, t37256, t37257, t37260)
}

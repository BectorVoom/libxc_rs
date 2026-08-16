//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1132/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1132(t1089: f64, t33304: f64, t3322: f64, t33494: f64, t3330: f64, t33312: f64, t11808: f64, t30187: f64, t3131: f64, t5658: f64, t1084: f64, t29568: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34050 = t33304 * t1089;
    let t34052 = t33494 * t3322;
    let t34054 = t33312 * t3330;
    let t34056 = t11808 * t30187;
    let t34058 = t3131 * t5658;
    let t34060 = t1084 * t34058 * t29568;
    (t34050, t34052, t34054, t34056, t34058, t34060)
}

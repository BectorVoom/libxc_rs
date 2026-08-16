//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 743/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk743(t1030: f64, t8686: f64, t1040: f64, t2974: f64, t3064: f64, t2973: f64, t2979: f64, t5987: f64, t2983: f64, t1: f64, t118: f64, t3: f64) -> (f64, f64, f64, f64, f64) {
    let t8798 = t1030 * t8686;
    let t8799 = t8798 * t1040;
    let t8801 = t3064 * t2974;
    let t8802 = t2973 * t8801;
    let t8804 = t5987 * t2979;
    let t8805 = t8804 * t2983;
    let t8808 = t118 * t1 * t3;
    (t8798, t8799, t8802, t8805, t8808)
}

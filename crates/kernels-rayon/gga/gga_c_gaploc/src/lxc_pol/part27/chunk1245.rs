//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1245/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1245(t24321: f64, t787: f64, t9824: f64, t1445: f64, t32223: f64, t833: f64, t32219: f64, t2615: f64, t32514: f64, t326: f64, t14667: f64, t2365: f64, t25289: f64) -> (f64, f64, f64, f64, f64) {
    let t33023 = t787 * t24321 * t9824;
    let t33024 = 0.14896037479937677779e-1_f64 * t33023;
    let t33030 = 0.11502877786176224903e2_f64 * t833 * t1445 * t32223;
    let t33033 = 0.23005755572352449806e2_f64 * t833 * t1445 * t32219;
    let t33041 = 0.18404604457881959845e2_f64 * t2615 * t326 * t32514;
    let t33047 = t14667 * t2365 * t25289;
    (t33024, t33030, t33033, t33041, t33047)
}

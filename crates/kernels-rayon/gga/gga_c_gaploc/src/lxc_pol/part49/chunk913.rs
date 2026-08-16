//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 913/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk913(t41738: f64, t6716: f64, t6717: f64, t12875: f64, t18651: f64, t40103: f64, t10526: f64, t20471: f64, t10241: f64, t2293: f64) -> (f64, f64, f64, f64, f64) {
    let t41741 = 0.62115540045351614476e2_f64 * t6716 * t6717 * t41738;
    let t41743 = 0.27606906686822939767e2_f64 * t18651 * t12875;
    let t41744 = 0.23005755572352449806e1_f64 * t40103;
    let t41747 = 0.21450293971110256001e1_f64 * t20471 * t10526 * t41738;
    let t41749 = t10241 * t2293;
    (t41741, t41743, t41744, t41747, t41749)
}

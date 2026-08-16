//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1139/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1139(t1445: f64, t47322: f64, t807: f64, t41411: f64, t43986: f64, t43989: f64, t43991: f64, t43993: f64, t43994: f64, t43997: f64, t44002: f64, t44005: f64, t44010: f64, t44012: f64) -> f64 {
    let t47462 = 0.23005755572352449806e1_f64 * t807 * t1445 * t47322;
    let t47463 = 0.51123901271894332903e0_f64 * t41411;
    let t47466 = t43986 - t43989 + 0.14896037479937677779e-1_f64 * t43991 + t47462 - t43993 - t43994 - t47463 - 0.11502877786176224903e2_f64 * t43997 + t44002 + t44005 + t44010 + 0.29792074959875355558e-1_f64 * t44012;
    t47466
}

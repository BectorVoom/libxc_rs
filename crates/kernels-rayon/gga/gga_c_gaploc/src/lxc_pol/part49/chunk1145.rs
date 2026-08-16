//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1145/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1145(t12256: f64, t9972: f64, t41425: f64, t44110: f64, t44111: f64, t44112: f64, t47506: f64, t47509: f64, t47511: f64, t47512: f64, t47513: f64, t47515: f64, t47517: f64) -> f64 {
    let t47519 = t12256 * t9972;
    let t47521 = 0.42603251059911944084e-1_f64 * t47506 + 0.14896037479937677779e-1_f64 * t47509 + t47511 + t47512 - t47513 + 0.10224780254378866581e1_f64 * t41425 + t47515 + t44110 - t44111 + t44112 + 0.29792074959875355558e-1_f64 * t47517 - 0.10725146985555128001e1_f64 * t47519;
    t47521
}

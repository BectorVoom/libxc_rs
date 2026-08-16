//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1190/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1190(t40261: f64, t38688: f64, t895: f64, t13814: f64, t4953: f64, t1445: f64, t1562: f64, t38613: f64, t874: f64, t40245: f64, t41974: f64, t41975: f64, t41976: f64, t41978: f64, t41979: f64, t41980: f64, t41982: f64, t41983: f64) -> f64 {
    let t47994 = 0.12780975317973583226e0_f64 * t40261;
    let t47995 = t895 * t38688;
    let t47997 = t4953 * t13814;
    let t48001 = t1562 * t1445 * t38613 * t874;
    let t48003 = t41974 - t41975 - t41976 - 0.76685851907841499354e0_f64 * t40245 + t41978 + t41979 - t41980 - t47994 + 0.23833659967900284446e0_f64 * t47995 - 0.69017266717057349418e1_f64 * t47997 - 0.69017266717057349418e1_f64 * t48001 + t41982 - t41983;
    t48003
}

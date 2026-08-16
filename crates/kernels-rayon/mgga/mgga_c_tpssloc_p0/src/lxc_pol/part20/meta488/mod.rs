//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1979;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1980;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta488(t2371: f64, t5154: f64, t12134: f64, t12136: f64, t12138: f64, t5151: f64, t67: f64, t758: f64, t12142: f64, t12127: f64, t12133: f64, t12141: f64, t15980: f64, t15983: f64, t15985: f64, t15987: f64, t15988: f64, t9853: f64, t9859: f64, t16160: f64, t16161: f64, t16163: f64, t225: f64, t1345: f64, t68: f64, t1799: f64, t1995: f64, t3734: f64, t1365: f64, t5187: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16165, t16166, t16167, t16168, t16169, t16171, t16172, t16173) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1979(t2371, t5154, t12134, t12136, t12138, t5151, t67, t758, t12142, t12127, t12133, t12141, t15980, t15983, t15985, t15987, t15988, t9853, t9859);
        let (t16176, t16186, t16191, t16192, t16195) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1980(t16160, t16161, t16163, t16173, t225, t1345, t68, t1799, t1995, t3734, t1365, t5187);
    (t16165, t16166, t16167, t16168, t16169, t16171, t16172, t16176, t16186, t16191, t16192, t16195)
}

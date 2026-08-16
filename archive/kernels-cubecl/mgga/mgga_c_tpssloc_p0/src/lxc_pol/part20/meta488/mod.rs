//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1979;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1980;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta488<F: Float>(t2371: F, t5154: F, t12134: F, t12136: F, t12138: F, t5151: F, t67: F, t758: F, t12142: F, t12127: F, t12133: F, t12141: F, t15980: F, t15983: F, t15985: F, t15987: F, t15988: F, t9853: F, t9859: F, t16160: F, t16161: F, t16163: F, t225: F, t1345: F, t68: F, t1799: F, t1995: F, t3734: F, t1365: F, t5187: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16165, t16166, t16167, t16168, t16169, t16171, t16172, t16173) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1979::<F>(t2371, t5154, t12134, t12136, t12138, t5151, t67, t758, t12142, t12127, t12133, t12141, t15980, t15983, t15985, t15987, t15988, t9853, t9859);
        let (t16176, t16186, t16191, t16192, t16195) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1980::<F>(t16160, t16161, t16163, t16173, t225, t1345, t68, t1799, t1995, t3734, t1365, t5187);
    (t16165, t16166, t16167, t16168, t16169, t16171, t16172, t16176, t16186, t16191, t16192, t16195)
}

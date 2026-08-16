//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta301 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1086;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1087;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta301(t16100: f64, t205: f64, t1345: f64, t68: f64, t12418: f64, t820: f64, t12289: f64, t242: f64, t1336: f64, t3804: f64, t3788: f64, t836: f64, t3777: f64, t5245: f64, t3734: f64, t571: f64, t2319: f64, t576: f64, t1351: f64, t1372: f64, t154: f64, t2558: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16101, t16186, t16224, t16233, t16305, t16397) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1086(t16100, t205, t1345, t68, t12418, t820, t12289, t242, t1336, t3804, t3788, t836);
        let (t16398, t16401, t16490, t16535, t22694, t22715) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1087(t1336, t16397, t3777, t5245, t3734, t571, t2319, t576, t1351, t1372, t154, t2558);
    (t16101, t16186, t16224, t16233, t16305, t16398, t16401, t16490, t16535, t22694, t22715)
}

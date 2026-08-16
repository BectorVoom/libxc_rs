//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta283 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk978;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk979;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta283<F: Float>(t20936: F, t225: F, t237: F, t119: F, t20756: F, t210: F, t1484: F, t5544: F, t2701: F, t820: F, t20870: F, t819: F, t13283: F, t1512: F, t1516: F, t16872: F, t16976: F, t20904: F, t20908: F, t249: F, t4172: F, t5587: F, t5624: F, t5628: F, t817: F, t843: F, t9559: F, t9974: F) -> (F, F, F, F, F, F, F) {
        let (t20937, t20938, t20944, t20947, t20949, t20953) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk978::<F>(t20936, t225, t237, t119, t20756, t210, t1484, t5544, t2701, t820, t20870, t819);
        let t20958 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk979::<F>(t13283, t1512, t1516, t16872, t16976, t20904, t20908, t20938, t20944, t20949, t20953, t249, t4172, t5587, t5624, t5628, t817, t843, t9559, t9974);
    (t20937, t20938, t20944, t20947, t20949, t20953, t20958)
}

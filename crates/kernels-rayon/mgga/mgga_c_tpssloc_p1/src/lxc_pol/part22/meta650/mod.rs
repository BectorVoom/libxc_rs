//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta650 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2190;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2191;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta650(t16398: f64, t19890: f64, t12283: f64, t19972: f64, t16046: f64, t1814: f64, t12250: f64, t5286: f64, t1372: f64, t6414: f64, t1338: f64, t20009: f64, t19731: f64, t562: f64, t16576: f64, t751: f64, t2517: f64, t5520: f64, t17109: f64, t870: f64, t16689: f64, t2430: f64, t12945: f64, t4205: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57450, t57457, t57530, t57568, t57618, t57659) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2190(t16398, t19890, t12283, t19972, t16046, t1814, t12250, t5286, t1372, t6414, t1338, t20009);
        let (t57704, t57887, t57897, t57932, t57947, t57960) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2191(t19731, t562, t16576, t751, t2517, t5520, t17109, t870, t16689, t2430, t12945, t4205);
    (t57450, t57457, t57530, t57568, t57618, t57659, t57704, t57887, t57897, t57932, t57947, t57960)
}

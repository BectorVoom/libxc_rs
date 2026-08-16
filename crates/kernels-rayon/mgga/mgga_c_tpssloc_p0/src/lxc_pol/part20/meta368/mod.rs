//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1712;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta368(t12813: f64, t510: f64, t1458: f64, t3652: f64, t4098: f64, t751: f64, t2752: f64, t4303: f64, t172: f64, t4095: f64, t763: f64, t1472: f64, t2517: f64, t40: f64, t1409: f64, t9427: f64, t2433: f64, t3966: f64, t12606: f64, t2244: f64, t2250: f64, t4080: f64, t607: f64, t73: f64, t9438: f64, t2440: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12835, t12841, t12850, t12854, t12858, t12860, t12861) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1712(t12813, t510, t1458, t3652, t4098, t751, t2752, t4303, t172, t4095, t763, t1472, t2517);
        let (t12862, t12865, t12873, t12874, t12877) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1713(t40, t1409, t9427, t2433, t3966, t12606, t2244, t2250, t4080, t607, t73, t9438, t2440, zeta_threshold);
    (t12835, t12841, t12850, t12854, t12858, t12860, t12861, t12862, t12865, t12873, t12874, t12877)
}

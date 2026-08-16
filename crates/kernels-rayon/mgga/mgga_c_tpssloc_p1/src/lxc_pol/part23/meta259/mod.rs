//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta259 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk920;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta259(t118: f64, t6330: f64, t794: f64, t12202: f64, t6347: f64, t3739: f64, t12211: f64, t6353: f64, t213: f64, t3726: f64, t6358: f64, t1814: f64, t5343: f64, t6378: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19767, t19768, t19775, t19776, t19779, t19781, t19791, t19810) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk920(t118, t6330, t794, t12202, t6347, t3739, t12211, t6353, t213, t3726, t6358, t1814, t5343);
        let t19815 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk921(t6378, t68);
    (t19767, t19768, t19775, t19776, t19779, t19781, t19791, t19810, t19815)
}

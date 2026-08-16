//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta674 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2262;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2263;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta674(t24987: f64, t6880: f64, t22573: f64, t7684: f64, t22575: f64, t22585: f64, t7685: f64, t22607: f64, t7754: f64, t6875: f64, t8944: f64, t26164: f64, t1983: f64, t22578: f64, t7753: f64, t7756: f64, t531: f64, t7752: f64, t22596: f64, t16153: f64, t24995: f64, t8945: f64, t22574: f64, t25988: f64, t31035: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91642, t91657, t91662, t91666, t91671) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2262(t24987, t6880, t22573, t7684, t22575, t22585, t7685, t22607, t7754, t6875, t8944, t26164);
        let (t91673, t91674, t91678, t91681, t91684) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2263(t1983, t22578, t7753, t22607, t7756, t531, t7752, t22596, t16153, t24995, t8945, t22574, t25988, t31035);
    (t91642, t91657, t91662, t91666, t91671, t91673, t91674, t91678, t91681, t91684)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1677;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1678;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta358(t12168: f64, t1343: f64, t820: f64, t3799: f64, t3858: f64, t12267: f64, t1340: f64, t120: f64, t3850: f64, t3805: f64, t3807: f64, t3719: f64, t550: f64, t3806: f64, t1352: f64, t5248: f64, t1995: f64, t67: f64, t246: f64, t3734: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12392, t12395, t12397, t12402, t12404, t12407) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1677(t12168, t1343, t820, t3799, t3858, t12267, t1340, t120, t3850, t3805, t3807, t3719, t550);
        let (t12409, t12413, t12418, t12419, t12420) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1678(t12407, t3805, t3806, t12402, t1352, t5248, t1995, t67, t246, t3734, t550);
    (t12392, t12395, t12397, t12404, t12407, t12409, t12413, t12418, t12419, t12420)
}

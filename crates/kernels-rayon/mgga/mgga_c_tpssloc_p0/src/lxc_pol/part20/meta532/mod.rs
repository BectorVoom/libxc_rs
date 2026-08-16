//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2067;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2068;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta532(t12250: f64, t3850: f64, t10021: f64, t154: f64, t59: f64, t3749: f64, t598: f64, t535: f64, t795: f64, t215: f64, t39933: f64, t12227: f64, t9577: f64, t116: f64, t557: f64, t212: f64, t2586: f64, t3734: f64, t12225: f64, t3719: f64, t12222: f64, t16081: f64, t1314: f64, t9534: f64, t1307: f64, t133: f64, t6600: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40335, t40341, t40343, t40344, t40347, t40350, t40351) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2067(t12250, t3850, t10021, t154, t59, t3749, t598, t535, t795, t215, t39933, t12227, t9577);
        let (t40356, t40360, t40366, t40369, t40372) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2068(t116, t557, t212, t2586, t3734, t12225, t3719, t12222, t16081, t1314, t9534, t1307, t133, t6600);
    (t40335, t40341, t40343, t40344, t40347, t40350, t40351, t40356, t40360, t40366, t40369, t40372)
}

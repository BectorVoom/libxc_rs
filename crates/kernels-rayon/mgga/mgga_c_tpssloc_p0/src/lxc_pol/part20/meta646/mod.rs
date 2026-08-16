//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta646 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2373;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2374;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta646(t291: f64, t48702: f64, t48722: f64, t10709: f64, t4483: f64, t10661: f64, t10662: f64, t1557: f64, t10817: f64, t14382: f64, t14385: f64, t42143: f64, t10655: f64, t14392: f64, t14396: f64, t42023: f64, t2792: f64, t2836: f64, t4396: f64, t14388: f64, t2793: f64, t10696: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48725, t48727, t48730, t48732, t48734) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2373(t291, t48702, t48722, t10709, t4483, t10661, t10662, t1557, t10817, t14382, t14385, t42143);
        let (t48736, t48738, t48741, t48744, t48747) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2374(t10655, t14392, t14396, t42023, t2792, t2836, t4396, t10661, t14388, t2793, t10696, t1557);
    (t48725, t48727, t48730, t48732, t48734, t48736, t48738, t48741, t48744, t48747)
}

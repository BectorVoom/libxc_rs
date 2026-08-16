//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta760 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2560;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2561;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta760(t4745: f64, t64257: f64, t4786: f64, t63755: f64, t14838: f64, t18255: f64, t14850: f64, t18259: f64, t11303: f64, t1136: f64, t11361: f64, t11420: f64, t15146: f64, t15207: f64, t1683: f64, t1694: f64, t18615: f64, t18623: f64, t18631: f64, t18634: f64, t18893: f64, t21839: f64, t21842: f64, t21952: f64, t3332: f64, t3357: f64, t3401: f64, t4819: f64, t4820: f64, t4857: f64, t51376: f64, t6037: f64, t6052: f64, t63533: f64, t18683: f64, t51249: f64, t18262: f64, t18266: f64, t51120: f64, t1117: f64, t11275: f64, t21961: f64, t11190: f64, t4781: f64, t6024: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71784, t71786, t71788, t71790, t71791) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2560(t4745, t64257, t4786, t63755, t14838, t18255, t14850, t18259, t11303, t1136, t11361, t11420, t15146, t15207, t1683, t1694, t18615, t18623, t18631, t18634, t18893, t21839, t21842, t21952, t3332, t3357, t3401, t4819, t4820, t4857, t51376, t6037, t6052, t63533);
        let (t71793, t71795, t71797, t71800, t71803) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2561(t18683, t51249, t14850, t18262, t18266, t51120, t1117, t11275, t21961, t11190, t4781, t6024);
    (t71784, t71786, t71788, t71790, t71791, t71793, t71795, t71797, t71800, t71803)
}

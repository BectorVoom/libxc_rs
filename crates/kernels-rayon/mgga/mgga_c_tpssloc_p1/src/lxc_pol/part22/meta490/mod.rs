//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1911;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1912;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1913;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1914;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta490(t1557: f64, t5726: f64, t2792: f64, t1556: f64, t17520: f64, t2842: f64, t1569: f64, t5758: f64, t10636: f64, t13598: f64, t17149: f64, t17165: f64, t17175: f64, t21124: f64, t21128: f64, t21147: f64, t21150: f64, t21153: f64, t21156: f64, t291: f64, t10608: f64, t324: f64, t10832: f64, t14276: f64, t21259: f64, t21263: f64, t21265: f64, t21267: f64, t21270: f64, t21302: f64, t21305: f64, t21306: f64, t21309: f64, t21312: f64, t2861: f64, t2886: f64, t2905: f64, t2930: f64, t311: f64, t5743: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21315, t21317, t21318, t21320, t21321, t21334) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1911(t1557, t5726, t2792, t1556, t17520, t2842, t1569, t5758, t10636, t13598, t17149, t17165, t17175, t21124, t21128, t21147, t21150, t21153, t21156);
        let (t21336, t21347) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1912(t21334, t291, t10608, t13598, t17149, t17165, t17175, t21124, t21128, t21147, t21150, t21153, t21156);
        let (t21348, t21360) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1913(t21347, t324, t10832, t13598, t17149, t17165, t17175, t21124, t21128, t21147, t21150, t21153, t21156);
        let t21363 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1914(t14276, t21259, t21263, t21265, t21267, t21270, t21302, t21305, t21306, t21309, t21312, t21317, t21320, t21321, t21336, t21348, t21360, t2861, t2886, t2905, t2930, t311, t5743);
    (t21315, t21317, t21318, t21320, t21321, t21334, t21336, t21347, t21348, t21360, t21363)
}

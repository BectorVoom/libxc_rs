//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta388 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1385;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1386;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1387;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta388(t300: f64, t5769: f64, t961: f64, t2904: f64, t5790: f64, t952: f64, t959: f64, t14473: f64, t1589: f64, t4483: f64, t4493: f64, t4489: f64, t10523: f64, t5774: f64, t4497: f64, t4472: f64, t4488: f64, t2929: f64, t17490: f64, t17504: f64, t17506: f64, t17509: f64, t17512: f64, t17515: f64, t17519: f64, t17523: f64, t17526: f64, t17530: f64, t17933: f64, t360: f64, t1021: f64, t248: f64, t1020: f64, t10413: f64, t10891: f64, t10949: f64, t14077: f64, t14080: f64, t14136: f64, t14139: f64, t14207: f64, t1618: f64, t1622: f64, t17907: f64, t17920: f64, t17925: f64, t3048: f64, t3070: f64, t4641: f64, t4652: f64, t5857: f64, t5875: f64, t5880: f64, t5900: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17936, t17940, t17942, t17944, t17946) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1385(t300, t5769, t961, t2904, t5790, t952, t959, t14473, t1589, t4483, t4493, t4489);
        let (t17950, t17953, t17957, t17958) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1386(t10523, t5774, t4497, t959, t4472, t4488, t2929, t5790, t17490, t17504, t17506, t17509, t17512, t17515, t17519, t17523, t17526, t17530, t17936, t17940, t17942, t17944, t17946);
        let (t17959, t17967) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1387(t17933, t17958, t360, t1021, t248, t1020, t10413, t10891, t10949, t14077, t14080, t14136, t14139, t14207, t1618, t1622, t17907, t17920, t17925, t3048, t3070, t4641, t4652, t5857, t5875, t5880, t5900);
    (t17936, t17940, t17942, t17944, t17946, t17950, t17953, t17957, t17959, t17967)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta272 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1433;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1434;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta272(t10422: f64, t3072: f64, t3070: f64, t3120: f64, t376: f64, t4594: f64, t4582: f64, t10283: f64, t10361: f64, t10364: f64, t10367: f64, t10370: f64, t10372: f64, t10377: f64, t10378: f64, t10381: f64, t10385: f64, t10388: f64, t10390: f64, t10394: f64, t10398: f64, t10403: f64, t10405: f64, t10410: f64, t10413: f64, t10415: f64, t10419: f64, t3073: f64, t3130: f64, t350: f64, t378: f64, t973: f64, t1023: f64, t1005: f64, t3082: f64, t1004: f64, t3088: f64, t1036: f64, t3094: f64, t1929: f64, t35: f64, t364: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10423, t10424, t10426, t10427, t10428, t10431) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1433(t10422, t3072, t3070, t3120, t376, t4594, t4582, t10283, t10361, t10364, t10367, t10370, t10372, t10377, t10378, t10381, t10385, t10388, t10390, t10394, t10398, t10403, t10405, t10410, t10413, t10415, t10419, t3073, t3130, t350, t378, t973);
        let (t10432, t10433, t10436, t10438, t10441, t10444, t10445) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1434(t1023, t10426, t4582, t1005, t3082, t1004, t3088, t1036, t3094, t1929, t35, t364);
    (t10423, t10424, t10427, t10428, t10431, t10432, t10433, t10436, t10438, t10441, t10444, t10445)
}

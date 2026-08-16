//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta272 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1433;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1434;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta272<F: Float>(t10422: F, t3072: F, t3070: F, t3120: F, t376: F, t4594: F, t4582: F, t10283: F, t10361: F, t10364: F, t10367: F, t10370: F, t10372: F, t10377: F, t10378: F, t10381: F, t10385: F, t10388: F, t10390: F, t10394: F, t10398: F, t10403: F, t10405: F, t10410: F, t10413: F, t10415: F, t10419: F, t3073: F, t3130: F, t350: F, t378: F, t973: F, t1023: F, t1005: F, t3082: F, t1004: F, t3088: F, t1036: F, t3094: F, t1929: F, t35: F, t364: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10423, t10424, t10426, t10427, t10428, t10431) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1433::<F>(t10422, t3072, t3070, t3120, t376, t4594, t4582, t10283, t10361, t10364, t10367, t10370, t10372, t10377, t10378, t10381, t10385, t10388, t10390, t10394, t10398, t10403, t10405, t10410, t10413, t10415, t10419, t3073, t3130, t350, t378, t973);
        let (t10432, t10433, t10436, t10438, t10441, t10444, t10445) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1434::<F>(t1023, t10426, t4582, t1005, t3082, t1004, t3088, t1036, t3094, t1929, t35, t364);
    (t10423, t10424, t10427, t10428, t10431, t10432, t10433, t10436, t10438, t10441, t10444, t10445)
}

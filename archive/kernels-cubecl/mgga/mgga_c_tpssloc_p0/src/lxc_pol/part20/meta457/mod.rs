//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1916;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1917;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1918;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta457<F: Float>(t14753: F, t4908: F, t14744: F, t11588: F, t1714: F, t3451: F, t3447: F, t14818: F, t14781: F, t14710: F, t11211: F, t11213: F, t11215: F, t11217: F, t11487: F, t14713: F, t14766: F, t14779: F, t14784: F, t14787: F, t14790: F, t14793: F, t14796: F, t14799: F, t457: F, t460: F, t974: F, t1716: F, t698: F, t1174: F, t3435: F, t4889: F, t135: F, t4930: F, t1420: F, t1887: F, t337: F, t11593: F, t4904: F, t11570: F, t3961: F, t11569: F, t3452: F, t3472: F, t3478: F) -> (F, F, F, F, F, F) {
        let (t15332, t15335, t15338, t15341, t15357) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1916::<F>(t14753, t4908, t14744, t11588, t1714, t3451, t3447, t14818, t14781, t14710, t11211, t11213, t11215, t11217, t11487, t14713, t14766, t14779, t14784, t14787, t14790, t14793, t14796, t14799);
        let (t15359, t15360, t15364, t15366, t15374, t15376) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1917::<F>(t15357, t457, t460, t974, t1716, t698, t1174, t3435, t4889, t135, t4930, t1420, t1887, t337);
        let (t15382, t15386) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1918::<F>(t11593, t4904, t11570, t3961, t11569, t1174, t15332, t15335, t15341, t15360, t15364, t15366, t15374, t15376, t3447, t3452, t3472, t3478, t4889);
    (t15338, t15357, t15359, t15376, t15382, t15386)
}

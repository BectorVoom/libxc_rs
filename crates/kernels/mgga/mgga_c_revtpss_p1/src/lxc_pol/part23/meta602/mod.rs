//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2253;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2254;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta602<F: Float>(t1089: F, t1678: F, t6299: F, t23820: F, t378: F, t6305: F, t3304: F, t1668: F, t6343: F, t12052: F, t24078: F, t23837: F, t1024: F, t1087: F, t12047: F, t12078: F, t12149: F, t12167: F, t15670: F, t16509: F, t1685: F, t19463: F, t24075: F, t24079: F, t24084: F, t24090: F, t24093: F, t24098: F, t3204: F, t3299: F, t4857: F, t4954: F, t4981: F, t4996: F, t6362: F, t6371: F, t6375: F, t6379: F, t6383: F) -> (F, F, F, F, F, F, F, F) {
        let (t24104, t24108, t24111, t24112, t24116, t24123, t24126) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2253::<F>(t1089, t1678, t6299, t23820, t378, t6305, t3304, t1668, t6343, t12052, t24078, t23837);
        let t24129 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2254::<F>(t1024, t1087, t12047, t12078, t12149, t12167, t15670, t16509, t1685, t19463, t24075, t24079, t24084, t24090, t24093, t24098, t24104, t24108, t24112, t24116, t24123, t24126, t3204, t3299, t4857, t4954, t4981, t4996, t6362, t6371, t6375, t6379, t6383);
    (t24104, t24108, t24111, t24112, t24116, t24123, t24126, t24129)
}

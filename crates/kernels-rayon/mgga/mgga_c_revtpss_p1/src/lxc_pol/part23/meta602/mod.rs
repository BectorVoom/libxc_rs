//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2253;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2254;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta602(t1089: f64, t1678: f64, t6299: f64, t23820: f64, t378: f64, t6305: f64, t3304: f64, t1668: f64, t6343: f64, t12052: f64, t24078: f64, t23837: f64, t1024: f64, t1087: f64, t12047: f64, t12078: f64, t12149: f64, t12167: f64, t15670: f64, t16509: f64, t1685: f64, t19463: f64, t24075: f64, t24079: f64, t24084: f64, t24090: f64, t24093: f64, t24098: f64, t3204: f64, t3299: f64, t4857: f64, t4954: f64, t4981: f64, t4996: f64, t6362: f64, t6371: f64, t6375: f64, t6379: f64, t6383: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24104, t24108, t24111, t24112, t24116, t24123, t24126) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2253(t1089, t1678, t6299, t23820, t378, t6305, t3304, t1668, t6343, t12052, t24078, t23837);
        let t24129 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2254(t1024, t1087, t12047, t12078, t12149, t12167, t15670, t16509, t1685, t19463, t24075, t24079, t24084, t24090, t24093, t24098, t24104, t24108, t24112, t24116, t24123, t24126, t3204, t3299, t4857, t4954, t4981, t4996, t6362, t6371, t6375, t6379, t6383);
    (t24104, t24108, t24111, t24112, t24116, t24123, t24126, t24129)
}

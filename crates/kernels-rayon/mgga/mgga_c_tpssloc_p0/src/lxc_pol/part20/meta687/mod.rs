//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta687 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2602;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2603;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2604;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta687(t11702: f64, t5002: f64, t11708: f64, t15502: f64, t15506: f64, t13969: f64, t15554: f64, t3506: f64, t10469: f64, t1720: f64, t10471: f64, t11737: f64, t11651: f64, t15507: f64, t11709: f64, t1174: f64, t11741: f64, t1177: f64, t11805: f64, t11809: f64, t15622: f64, t15627: f64, t15631: f64, t1737: f64, t44858: f64, t44896: f64, t45080: f64, t4582: f64, t4978: f64, t5005: f64, t50865: f64, t50869: f64, t52659: f64, t15621: f64, t11791: f64, t11697: f64, t15477: f64, t3577: f64, t11677: f64, t15027: f64, t11680: f64, t11684: f64, t11751: f64, t1227: f64, t15740: f64, t3440: f64, t45997: f64, t4889: f64, t4972: f64, t50873: f64, t50884: f64, t50959: f64, t50964: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52801, t52810, t52813, t52817, t52834, t52835, t52836) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2602(t11702, t5002, t11708, t15502, t15506, t13969, t15554, t3506, t10469, t1720, t10471, t11737);
        let t52853 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2603(t11651, t15507, t11709, t1174, t11741, t1177, t11805, t11809, t15622, t15627, t15631, t1737, t3506, t44858, t44896, t45080, t4582, t4978, t5005, t50865, t50869, t52659, t52836);
        let t52886 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2604(t13969, t15621, t3506, t11791, t5005, t11697, t15477, t3577, t11677, t15027, t11680, t11684, t1174, t11751, t1177, t1227, t15740, t3440, t4582, t45997, t4889, t4972, t50873, t50884, t50959, t50964);
    (t52801, t52810, t52813, t52817, t52834, t52835, t52853, t52886)
}

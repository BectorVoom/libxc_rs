//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2185;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2186;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta604(t11719: f64, t11722: f64, t248: f64, t3570: f64, t11818: f64, t1213: f64, t3494: f64, t3506: f64, t3509: f64, t3515: f64, t3516: f64, t11718: f64, t44857: f64, t11661: f64, t13969: f64, t11721: f64, t3493: f64, t11858: f64, t1226: f64, t3030: f64, t3481: f64, t3032: f64, t3505: f64, t3514: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44871, t44886, t44890, t44894, t44896) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2185(t11719, t11722, t248, t3570, t11818, t1213, t3494, t3506, t3509, t3515, t3516, t11718, t44857);
        let (t44904, t44906, t44918, t44927, t44929, t44932) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2186(t11661, t13969, t3506, t11721, t3493, t11858, t1226, t3030, t3481, t3032, t3505, t3514);
    (t44871, t44886, t44890, t44894, t44896, t44904, t44906, t44918, t44927, t44929, t44932)
}

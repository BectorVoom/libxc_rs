//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta534 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2071;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta534(t12204: f64, t40409: f64, t12214: f64, t792: f64, t118: f64, t12156: f64, t794: f64, t2229: f64, t59: f64, t60: f64, t535: f64, t9538: f64, t12231: f64, t3726: f64, t12199: f64, t12208: f64, t12012: f64, t3739: f64, t12217: f64, t40021: f64, t3774: f64, t3862: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40410, t40415, t40419, t40422) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2071(t12204, t40409, t12214, t792, t118, t12156, t794, t2229, t59, t60, t535, t9538);
        let (t40423, t40425, t40429, t40431, t40443) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2072(t12231, t3726, t12199, t12208, t118, t12012, t3739, t794, t12217, t40021, t3774, t3862);
    (t40410, t40415, t40419, t40422, t40423, t40425, t40429, t40431, t40443)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2549/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2549(t11821: f64, t140: f64, t42793: f64, t4892: f64, t4895: f64, t4899: f64, t4901: f64, t1011: f64, t1655: f64, t2438: f64, t1014: f64, t4579: f64, t697: f64) -> (f64, f64, f64, f64, f64) {
    let t53972 = t140 * t11821;
    let t54036 = t4892 * t42793 * t4895;
    let t54037 = 0.28582678745379824648e-3_f64 * t54036;
    let t54078 = t4899 * t42793 * t4901;
    let t54079 = 0.14291339372689912324e-3_f64 * t54078;
    let t54118 = t1011 * t2438 * t1655;
    let t54122 = t1011 * t697 * t1014 * t4579;
    (t53972, t54037, t54079, t54118, t54122)
}

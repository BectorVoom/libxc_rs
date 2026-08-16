//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1446/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1446(t12166: f64, t12177: f64, t12182: f64, t12188: f64, t12207: f64, t1589: f64, t2043: f64, t2049: f64, t2052: f64, t29009: f64, t29011: f64, t29014: f64, t29016: f64, t29019: f64, t29023: f64, t29025: f64, t29032: f64, t29035: f64, t3732: f64, t3740: f64, t4598: f64, t6096: f64, t784: f64, t797: f64, t813: f64) -> f64 {
    let t39321 = t29009 - t29011 - t29014 + t29016 - t29019 - t29023 + t29025 + t29032 - t29035 + 0.35750489951850426669e0_f64 * t2043 * t12207 + 0.71500979903700853338e0_f64 * t2052 * t3732 * t6096 + 0.47667319935800568892e0_f64 * t12182 * t784 + 0.47667319935800568892e0_f64 * t12177 * t784 - 0.47667319935800568892e0_f64 * t2049 * t12188 - 0.47667319935800568892e0_f64 * t797 * t1589 * t12166 - 0.1022478025437886658e1_f64 * t813 * t4598 * t3740;
    t39321
}

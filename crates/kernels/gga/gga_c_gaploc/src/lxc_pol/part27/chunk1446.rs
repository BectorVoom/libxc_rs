//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1446/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1446<F: Float>(t12166: F, t12177: F, t12182: F, t12188: F, t12207: F, t1589: F, t2043: F, t2049: F, t2052: F, t29009: F, t29011: F, t29014: F, t29016: F, t29019: F, t29023: F, t29025: F, t29032: F, t29035: F, t3732: F, t3740: F, t4598: F, t6096: F, t784: F, t797: F, t813: F) -> F {
    let t39321 = t29009 - t29011 - t29014 + t29016 - t29019 - t29023 + t29025 + t29032 - t29035 + F::cast_from(0.35750489951850426669e0_f64) * t2043 * t12207 + F::cast_from(0.71500979903700853338e0_f64) * t2052 * t3732 * t6096 + F::cast_from(0.47667319935800568892e0_f64) * t12182 * t784 + F::cast_from(0.47667319935800568892e0_f64) * t12177 * t784 - F::cast_from(0.47667319935800568892e0_f64) * t2049 * t12188 - F::cast_from(0.47667319935800568892e0_f64) * t797 * t1589 * t12166 - F::cast_from(0.1022478025437886658e1_f64) * t813 * t4598 * t3740;
    t39321
}

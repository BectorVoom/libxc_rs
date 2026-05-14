//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 863/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk863<F: Float>(t2009: F, t2021: F, t43932: F, t43683: F, t7584: F, t7585: F, t2639: F, t3431: F, t7284: F, t787: F, t13008: F, t2087: F, t4614: F, t13133: F, t2197: F, t43901: F, t43904: F, t43908: F, t43909: F, t43910: F, t43911: F, t43913: F, t43915: F, t43918: F, t43919: F, t43922: F, t43924: F, t43926: F, t43928: F, t43931: F) -> (F,) {
    let t43935 = 0.35750489951850426669e0 * t2021 * t43932 * t2009;
    let t43938 = 0.11502877786176224903e2 * t7584 * t7585 * t43683;
    let t43941 = t787 * t7284 * t3431 * t2639;
    let t43944 = t2087 * t4614 * t13008;
    let t43946 = t2197 * t13133;
    let t43948 = 0.47667319935800568892e0 * t43901 - 0.51123901271894332901e0 * t43904 + t43908 - t43909 + t43910 + t43911 - t43913 + t43915 + t43918 - 0.38342925953920749676e0 * t43919 - 0.38342925953920749676e0 * t43922 + t43924 - t43926 - t43928 - t43931 - t43935 - t43938 - 0.50050685932590597338e1 * t43941 - 0.18404604457881959845e2 * t43944 + 0.23005755572352449806e2 * t43946;
    (t43948,)
}

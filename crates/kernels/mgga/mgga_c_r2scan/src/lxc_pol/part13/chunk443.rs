//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 443/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk443<F: Float>(t1691: F, t721: F, t695: F, t713: F, t194: F, t685: F, t63: F, t1441: F, t1442: F, t1443: F, t1444: F, t1714: F, t1717: F) -> (F, F, F, F, F) {
    let t1946 = t721 * t1691;
    let t1949 = t713 * t695;
    let t1956 = F::new(1.0) / t685 / t194;
    let t1957 = t63 * t1956;
    let t1966 = -F::new(0.39219166666666666667e0) * t1714 + F::new(0.31375333333333333333e1) * t1717 + t1441 + t1442 + t1443 + t1444;
    (t1946, t1949, t1956, t1957, t1966)
}

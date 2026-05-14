//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 924/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk924<F: Float>(t1079: F, t2953: F, t1081: F, t2958: F, t2956: F, t1020: F, t1083: F, t1085: F, t1087: F, t1089: F, t1091: F, t3652: F, t3656: F, t3660: F, t3664: F, t3668: F) -> (F, F) {
    let t12629 = t1079 * t2953;
    let t12632 = t2958 * t1081;
    let t12654 = t2956 * t1081;
    let t12656 = -0.9214113627294e1 * t12632 - 0.18428227254588e2 * t3652 * t1020 - 0.9214113627294e1 * t1083 * t2956 + 0.734774460522e2 * t3656 * t1020 + 0.367387230261e2 * t1085 * t2956 - 0.7662840944824e2 * t3660 * t1020 - 0.3831420472412e2 * t1087 * t2956 + 0.3101306810232e2 * t3664 * t1020 + 0.1550653405116e2 * t1089 * t2956 - 0.4355305902528e1 * t3668 * t1020 - 0.2177652951264e1 * t1091 * t2956 - 0.8704e0 * t12654;
    (t12629, t12656)
}

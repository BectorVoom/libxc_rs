//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 826/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk826<F: Float>(t13589: F, t4083: F, t1255: F, t1264: F, t1276: F, t13512: F, t13518: F, t13557: F, t13566: F, t13571: F, t13574: F, t13578: F, t13583: F, t13588: F, t361: F, t4026: F, t4076: F, t4084: F, t4096: F, t4103: F) -> (F,) {
    let t13590 = t13589 * t4083;
    let t13593 = -0.17544670192365612213e1 * t13512 * t1276 + 0.35089340384731224426e1 * t4096 * t4103 - 0.51947267698127589897e2 * t1264 * t13518 - 0.62182e-1 * t13557 * t361 - 0.1025389702100779493e4 * t1264 * t13566 + 0.35089340384731224426e1 * t1264 * t13571 + 3.0 * t13574 * t1255 - 0.35089340384731224426e1 * t1264 * t13578 + 3.0 * t4026 * t4076 + 0.48245472966453314466e2 * t13583 * t4084 - 0.96490945932906628932e2 * t13588 * t13590;
    (t13593,)
}

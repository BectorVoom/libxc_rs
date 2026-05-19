//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1008/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1008<F: Float>(t240: F, t3688: F, t1213: F, t12889: F, t12893: F, t13050: F, t13053: F, t13056: F, t13060: F, t13066: F, t13105: F, t13123: F, t1550: F, t3718: F, t3726: F, t4486: F) -> F {
    let t14850 = t240 * t3688;
    let t14865 = -F::cast_from(0.17544670192365612213e1_f64) * t14850 * t1213 - F::cast_from(0.17544670192365612213e1_f64) * t4486 * t3718 - F::cast_from(0.1025389702100779493e4_f64) * t1550 * t12889 - F::cast_from(0.35089340384731224426e1_f64) * t1550 * t13105 - t13050 + t13053 - t13056 + t13060 - F::cast_from(0.51947267698127589897e2_f64) * t1550 * t12893 - F::cast_from(0.51947267698127589899e2_f64) * t4486 * t3726 + F::cast_from(0.1038945353962551798e3_f64) * t1550 * t13066 - t13123;
    t14865
}

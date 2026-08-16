//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 707/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk707<F: Float>(t1705: F, t4850: F, t10894: F, t10898: F, t10907: F, t10915: F, t10918: F, t10925: F, t10929: F, t10968: F, t1726: F, t1735: F, t1747: F, t45: F, t4924: F, t4931: F, t4950: F, t4958: F, t634: F) -> F {
    let t10972 = t4850 * t1705;
    let t10975 = F::cast_from(0.35089340384731224426e1_f64) * t4924 * t4931 + F::cast_from(0.35089340384731224426e1_f64) * t1735 * t10894 - F::cast_from(0.51947267698127589897e2_f64) * t1735 * t10898 - F::cast_from(0.1025389702100779493e4_f64) * t1735 * t10907 - F::cast_from(0.51947267698127589899e2_f64) * t4924 * t4958 + F::cast_from(0.1038945353962551798e3_f64) * t1735 * t10915 - F::cast_from(0.17544670192365612213e1_f64) * t10918 * t1747 - F::cast_from(0.17544670192365612213e1_f64) * t4924 * t4950 + F::cast_from(0.51725014705706168417e3_f64) * t10925 * t10929 + F::cast_from(0.19751789702565206229e-1_f64) * t45 * t10968 * t634 + F::cast_from(3.0_f64) * t10972 * t1726;
    t10975
}

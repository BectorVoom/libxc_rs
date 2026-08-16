//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2627/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2627<F: Float>(t18529: F, t4889: F, t1174: F, t135: F, t22034: F, t15338: F, t18409: F, t3447: F, t15320: F, t15376: F, t18427: F, t18434: F, t52058: F, t64711: F, t64713: F, t64718: F, t64730: F, t64733: F) -> F {
    let t73386 = t4889 * t18529;
    let t73389 = t1174 * t135 * t22034;
    let t73395 = t3447 * t15338 * t18409;
    let t73399 = -F::cast_from(0.14814814814814814814e-2_f64) * t64711 + F::cast_from(0.29629629629629629628e-2_f64) * t64713 + F::cast_from(0.27777777777777777777e-3_f64) * t64718 - F::cast_from(0.14814814814814814814e-2_f64) * t64730 + F::cast_from(0.37037037037037037036e-3_f64) * t64733 + t52058 + F::cast_from(0.22222222222222222222e-2_f64) * t73386 - F::cast_from(0.27777777777777777777e-3_f64) * t73389 + F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t15320 * t18427 + F::cast_from(0.27777777777777777777e-3_f64) * t73395 - F::cast_from(0.44444444444444444443e-2_f64) * t15376 * t18434;
    t73399
}

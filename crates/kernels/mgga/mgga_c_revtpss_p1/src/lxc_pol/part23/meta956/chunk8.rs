//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3199/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3199<F: Float>(t1219: F, t24551: F, t21254: F, t5373: F, t20858: F, t21200: F, t21203: F, t44931: F, t59144: F, t59411: F, t71470: F, t71476: F, t71490: F, t71539: F, t71541: F) -> F {
    let t84029 = t24551 * t1219;
    let t84032 = t5373 * t21254;
    let t84036 = F::cast_from(0.57165357490759649295e-3_f64) * t71470 + F::cast_from(0.45732285992607719436e-2_f64) * t71476 + F::cast_from(0.12862205435420921092e-2_f64) * t59411 * t20858 - F::cast_from(0.25724410870841842183e-2_f64) * t71490 - F::cast_from(0.13719685797782315831e-1_f64) * t21203 * t21200 - F::cast_from(5.0_f64) / F::cast_from(3888.0_f64) * t44931 - F::cast_from(77.0_f64) / F::cast_from(486.0_f64) * t84029 - F::cast_from(5.0_f64) / F::cast_from(162.0_f64) * t59144 + t84032 / F::cast_from(54.0_f64) + F::cast_from(0.28582678745379824648e-2_f64) * t71539 + F::cast_from(0.57165357490759649295e-3_f64) * t71541;
    t84036
}

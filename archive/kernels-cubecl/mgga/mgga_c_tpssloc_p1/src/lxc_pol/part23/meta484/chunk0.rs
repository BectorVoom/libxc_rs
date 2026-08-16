//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1472/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1472<F: Float>(t225: F, t78637: F, t11546: F, t1174: F, t15569: F, t15740: F, t1653: F, t1726: F, t22162: F, t22244: F, t22280: F, t22288: F, t3440: F, t3577: F, t3578: F, t45112: F, t484: F, t488: F, t52628: F, t52879: F, t53274: F, t66500: F, t68: F, t73043: F, t73113: F, t78035: F, t78039: F) -> (F, F) {
    let t79260 = t78637 * t225;
    let t79282 = -t15740 * t22288 / F::cast_from(192.0_f64) + t52628 * t22280 / F::cast_from(36.0_f64) - t52879 * t22280 / F::cast_from(192.0_f64) - t45112 + t79260 * t68 * t484 * t488 / F::cast_from(3072.0_f64) - F::cast_from(7.0_f64) / F::cast_from(108.0_f64) * t1174 * t11546 * t78035 + F::cast_from(154.0_f64) / F::cast_from(243.0_f64) * t73113 * t1726 + t1174 * t3440 * t78039 / F::cast_from(6.0_f64) - t53274 / F::cast_from(486.0_f64) + t73043 / F::cast_from(1152.0_f64) - t3577 * t3578 * t22244 * t1653 / F::cast_from(1152.0_f64) + t15569 * t22162 / F::cast_from(72.0_f64) - F::cast_from(11.0_f64) / F::cast_from(81.0_f64) * t66500;
    (t79260, t79282)
}

//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1228/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1228<F: Float>(t5895: F, t645: F, t18434: F, t18461: F, t18437: F, t18440: F, t18442: F, t18447: F, t18451: F, t18455: F, t18457: F, t18459: F, t18465: F, t18467: F, t18469: F) -> (F, F, F, F) {
    let t18930 = t5895 * t645;
    let t18934 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t18434;
    let t18943 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t18461;
    let t18947 = t18934 + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t18437 + t18440 / F::cast_from(8.0_f64) - t18442 / F::cast_from(24.0_f64) + t18447 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t18451 + t18455 / F::cast_from(96.0_f64) - t18457 / F::cast_from(768.0_f64) - t18459 / F::cast_from(768.0_f64) + t18943 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t18465 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t18467 - t18469 / F::cast_from(192.0_f64);
    (t18930, t18934, t18943, t18947)
}

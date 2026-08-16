//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1313/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1313<F: Float>(t1955: F, t23359: F, t106395: F, t106407: F, t106423: F, t106431: F, t106433: F, t1949: F, t1959: F, t231: F, t23384: F, t23413: F, t23414: F, t27199: F, t29655: F, t29675: F, t6016: F, t7053: F, t7070: F, t7076: F, t7759: F, t93118: F, t93334: F, t99425: F, t99435: F) -> F {
    let t113373 = t1955 * t23359;
    let t113380 = -F::cast_from(0.68549505033305214441e-2_f64) * t99425 - F::cast_from(0.65854491829355115987e0_f64) * t7053 * t23384 + F::cast_from(0.29272321618148349057e-1_f64) * t106395 + F::cast_from(0.10408353825846239354e2_f64) * t7070 * t93118 * t1949 * t23413 + F::cast_from(0.26020884564615598386e1_f64) * t27199 * t29655 + F::cast_from(0.34697458558045176417e-2_f64) * t99435 - F::cast_from(0.29272321618148349057e-1_f64) * t106407 + F::cast_from(0.13010442282307799193e1_f64) * t7070 * t7076 * t7759 * t6016 * t231 + F::cast_from(0.13010442282307799193e1_f64) * t27199 * t29675 + F::cast_from(0.32927245914677557992e-1_f64) * t106423 - F::cast_from(0.4336814094102599731e0_f64) * t113373 * t1959 - t93334 + F::cast_from(0.38554277296572111609e-1_f64) * t106431 - F::cast_from(0.21684070470512998656e-1_f64) * t106433 - F::cast_from(0.39512695097613069591e1_f64) * t7053 * t23414;
    t113380
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2154/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2154<F: Float>(t100981: F, t106565: F, t1113: F, t6079: F, t105930: F, t106487: F, t106496: F, t107924: F, t107927: F, t107930: F, t107934: F, t107939: F, t107943: F, t1940: F, t1963: F, t2403: F, t25206: F, t25440: F, t25445: F, t27158: F, t27368: F, t27382: F, t27764: F, t27802: F, t27806: F, t29970: F, t6416: F, t7087: F) -> F {
    let t107947 = t100981 * t106565;
    let t107958 = t1113 * t6079;
    let t107963 = t105930 - t106496 + F::cast_from(2.0_f64) * t27382 * t107924 - F::cast_from(6.0_f64) * t27158 * t107927 - F::cast_from(3.0_f64) * t25206 * t107930 + F::cast_from(6.0_f64) * t27158 * t107934 + F::cast_from(6.0_f64) * t106487 * t27764 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t1963 * t107939 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t1963 * t107943 - F::cast_from(3.0_f64) * t27382 * t107947 - t1940 * t27368 * t27806 + t1940 * t7087 * t6416 / F::cast_from(2.0_f64) - t1940 * t25440 * t29970 / F::cast_from(2.0_f64) + t1940 * t25445 * t107958 - t1940 * t27368 * t27802;
    t107963
}

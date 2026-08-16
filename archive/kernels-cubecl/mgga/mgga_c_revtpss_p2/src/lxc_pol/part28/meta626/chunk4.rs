//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2239/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2239<F: Float>(t14365: F, t14436: F, t14468: F, t14749: F, t14767: F, t1940: F, t1963: F, t198: F, t207: F, t2394: F, t2403: F, t2408: F, t25206: F, t25445: F, t27368: F, t27384: F, t4433: F, t4541: F, t61155: F, t61182: F, t63164: F, t7087: F, t7091: F, t7783: F, t892: F, t92742: F, t93404: F, t98722: F, t98759: F, t98779: F, t98786: F, t99536: F) -> F {
    let t100858 = t14436 * t14365;
    let t100882 = F::cast_from(12.0_f64) * t4541 * t7087 * t4433 + F::cast_from(2.0_f64) * t1940 * t98722 * t2408 - F::cast_from(6.0_f64) * t4541 * t7091 * t98759 - F::cast_from(6.0_f64) * t2403 * t27368 * t14365 + F::cast_from(4.0_f64) * t1940 * t93404 * t27384 + t198 * t207 * t99536 * t892 + F::cast_from(6.0_f64) * t2403 * t25445 * t61155 + F::cast_from(4.0_f64) * t1940 * t25445 * t63164 + F::cast_from(12.0_f64) * t25206 * t100858 - F::cast_from(6.0_f64) * t1940 * t92742 * t98786 + F::cast_from(3.0_f64) * t2403 * t1963 * t14468 + F::cast_from(6.0_f64) * t4541 * t7783 * t2394 - F::cast_from(6.0_f64) * t2403 * t7091 * t61182 + F::cast_from(2.0_f64) * t1940 * t25445 * t98779 + F::cast_from(12.0_f64) * t4541 * t1963 * t14749 + F::cast_from(6.0_f64) * t4541 * t1963 * t14767;
    t100882
}

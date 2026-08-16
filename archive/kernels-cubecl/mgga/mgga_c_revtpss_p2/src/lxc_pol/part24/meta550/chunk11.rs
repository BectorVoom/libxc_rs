//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1637/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1637<F: Float>(t18865: F, t1940: F, t198: F, t23279: F, t2403: F, t29598: F, t40067: F, t40072: F, t40167: F, t40171: F, t40184: F, t4541: F, t4546: F, t5962: F, t6079: F, t61033: F, t77333: F, t87670: F, t87671: F, t87673: F, t87674: F, t87675: F) -> F {
    let t87966 = -F::cast_from(36.0_f64) * t18865 * t2403 * t29598 + F::cast_from(12.0_f64) * t1940 * t6079 * t61033 + F::cast_from(36.0_f64) * t198 * t5962 * t77333 + F::cast_from(72.0_f64) * t23279 * t4541 * t4546 + t40067 - t40072 + t40167 - t40171 - t40184 - t87670 - t87671 + t87673 - t87674 + t87675;
    t87966
}

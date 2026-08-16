//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1806/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1806<F: Float>(t1832: F, t5023: F, t81139: F, t90351: F, t90356: F, t90361: F, t90364: F, t90367: F, t90370: F, t90373: F, t90375: F, t90377: F, t90503: F, t90505: F, t90509: F) -> F {
    let t91758 = -F::cast_from(4.0_f64) * t1832 * t5023 * t81139 + t90351 - t90356 - t90361 - t90364 - t90367 + t90370 + t90373 - t90375 - t90377 - t90503 + t90505 + t90509;
    t91758
}

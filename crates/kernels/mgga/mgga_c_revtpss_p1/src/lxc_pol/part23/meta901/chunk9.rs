//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2876/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2876<F: Float>(t262: F, t5966: F, t23148: F, t23124: F, t39429: F, t39432: F, t39442: F, t4541: F, t49877: F, t50080: F, t76937: F, t76938: F, t76939: F, t76940: F, t76941: F, t775: F) -> (F, F) {
    let t77333 = t5966 * t262;
    let t77341 = t262 * t23148;
    let t77347 = F::cast_from(6.0_f64) * t4541 * t77341 * t775 + F::cast_from(18.0_f64) * t23124 * t50080 - t39429 - t39432 + t39442 + t49877 - t76937 - t76938 - t76939 + t76940 + t76941;
    (t77333, t77347)
}

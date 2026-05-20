//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1695/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1695<F: Float>(t1668: F, t24031: F, t88004: F, t88007: F, t88012: F, t88016: F, t88023: F, t88026: F, t88028: F, t88030: F, t88034: F, t88036: F, t88038: F) -> (F, F) {
    let t88948 = t24031 * t1668;
    let t88980 = -t88004 + t88007 - t88012 + t88016 - t88023 + t88026 - t88028 - t88030 + t88034 - t88036 - t88038;
    (t88948, t88980)
}

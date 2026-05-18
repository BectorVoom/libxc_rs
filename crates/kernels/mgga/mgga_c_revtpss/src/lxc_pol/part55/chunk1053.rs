//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1053/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1053<F: Float>(t27383: F, t32505: F, t1962: F, t605: F, t1940: F, t198: F, t207: F, t2403: F, t26585: F, t26590: F, t32486: F, t32491: F, t32498: F, t7086: F, t7432: F, t775: F, t8657: F, t890: F, t892: F) -> (F, F, F) {
    let t32506 = t27383 * t32505;
    let t32508 = t605 * t1962;
    let t32534 = t198 * t207 * t32486 * t892 - t1940 * t1962 * t26585 + F::new(2.0) * t1940 * t26590 * t32505 - t1940 * t32491 * t890 - t1940 * t7086 * t7432 - F::new(3.0) * t2403 * t32498 * t7432 + F::new(3.0) * t2403 * t775 * t8657;
    (t32506, t32508, t32534)
}

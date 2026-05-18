//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 852/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk852<F: Float>(t25411: F, t25413: F, t1950: F, t2453: F, t2458: F, t25372: F, t25410: F, t2411: F, t7086: F, t11064: F, t1962: F, t1032: F, t1071: F) -> (F, F, F, F, F, F, F) {
    let t25414 = t25411 * t25413;
    let t25422 = t2453 * t1950;
    let t25424 = F::new(0.11565819519348392139e-2) * t25422 * t2458;
    let t25431 = t25372 * t25410;
    let t25432 = t25431 * t25413;
    let t25440 = t7086 * t2411;
    let t25445 = t1962 * t11064;
    let t25460 = t1071 * t1032;
    (t25414, t25424, t25431, t25432, t25440, t25445, t25460)
}

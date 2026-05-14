//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 514/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk514<F: Float>(t2848: F, t2906: F, t2994: F, t3001: F, t4571: F, t4576: F, t4581: F, t4585: F, t4599: F, t4607: F, t4615: F, t4617: F, t4620: F, t4623: F, t4626: F, t4629: F) -> (F,) {
    let t4707 = -0.1294625e1 * t4599 + 0.258925e1 * t4607 + t2994 + 0.10064166666666666667e0 * t2848 + 0.10064166666666666667e0 * t4571 - 0.20128333333333333333e0 * t4576 + 0.60385e0 * t4581 - 0.301925e0 * t4585 + 0.82524375e-1 * t4615 + 0.16504875e0 * t4617 + t3001 + 0.5519e-1 * t2906 + 0.5519e-1 * t4620 - 0.27595e-1 * t4623 + 0.16557e0 * t4626 - 0.82785e-1 * t4629;
    (t4707,)
}

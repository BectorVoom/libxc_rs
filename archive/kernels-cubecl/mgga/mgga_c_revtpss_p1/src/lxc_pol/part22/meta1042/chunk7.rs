//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3643/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3643<F: Float>(t58209: F, t58211: F, t58225: F, t68456: F, t68459: F, t68567: F, t68570: F, t68573: F, t68576: F, t68578: F, t68583: F, t68585: F, t68588: F, t68590: F, t68593: F) -> F {
    let t68936 = -F::cast_from(0.11958666666666666667e1_f64) * t68456 + F::cast_from(0.17938e1_f64) * t68459 - F::cast_from(0.10954222222222222222e0_f64) * t68567 + F::cast_from(0.82156666666666666667e-1_f64) * t68570 - F::cast_from(0.54771111111111111112e-1_f64) * t68573 - F::cast_from(0.27385555555555555556e-1_f64) * t68576 + F::cast_from(0.3071625e0_f64) * t68578 - F::cast_from(0.21908444444444444444e0_f64) * t58209 - F::cast_from(0.65725333333333333332e0_f64) * t58211 + F::cast_from(0.73028148148148148147e0_f64) * t58225 + F::cast_from(0.91285185185185185185e-1_f64) * t68583 + F::cast_from(0.18257037037037037037e0_f64) * t68585 + F::cast_from(0.32862666666666666666e0_f64) * t68588 - F::cast_from(0.30428395061728395062e-1_f64) * t68590 - F::cast_from(0.54771111111111111112e-1_f64) * t68593;
    t68936
}

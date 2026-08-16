//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1323/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1323<F: Float>(t33661: F, t28739: F, t28743: F, t33624: F, t33626: F, t33630: F, t33633: F, t33637: F, t33640: F, t33642: F, t33645: F, t33649: F, t33651: F, t33653: F, t33656: F, t33659: F) -> F {
    let t33662 = F::cast_from(0.85206502119823888168e-1_f64) * t33661;
    let t33663 = -t33624 - t33626 - t33630 - t28739 - t28743 + t33633 - t33637 - t33640 - t33642 - t33645 - t33649 - t33651 - t33653 + t33656 + t33659 - t33662;
    t33663
}

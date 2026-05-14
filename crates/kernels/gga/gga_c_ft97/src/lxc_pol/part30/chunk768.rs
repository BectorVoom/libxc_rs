//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 768/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk768<F: Float>(t35593: F, t35632: F, t35692: F, t35742: F, t35516: F, t675: F, t263: F, t193: F, t6743: F, t7150: F, t1091: F, t1425: F, t666: F, t461: F, t6903: F, t231: F, t6837: F) -> (F, F, F, F, F, F, F, F, F) {
    let t35744 = t35593 + t35632 + t35692 + t35742;
    let t35751 = t675 * t35516;
    let t35752 = t35751 * t263;
    let t35753 = t193 * t35752;
    let t35757 = t6743 * t7150;
    let t35760 = t1425 * t1091;
    let t35761 = t666 * t35760;
    let t35766 = t461 * t6903;
    let t35772 = t231 * t6837;
    (t35744, t35751, t35752, t35753, t35757, t35760, t35761, t35766, t35772)
}

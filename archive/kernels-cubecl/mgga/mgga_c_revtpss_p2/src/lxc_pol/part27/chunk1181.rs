//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1181/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1181<F: Float>(t2452: F, t9720: F, t675: F, t886: F, t11006: F, t256: F, t10115: F, t251: F, t2410: F, t2832: F, t775: F, t11238: F, t196: F) -> (F, F, F, F, F, F, F) {
    let t40688 = t9720 * t2452;
    let t41040 = t675 * t886;
    let t41077 = F::cast_from(1.0_f64) / t11006 / t256;
    let t41117 = t10115 * t251;
    let t41153 = t2410 * t2410;
    let t41154 = F::cast_from(1.0_f64) / t41153;
    let t41161 = t775 * t2832;
    let t42859 = F::cast_from(1.0_f64) / t11238 / t196;
    (t40688, t41040, t41077, t41117, t41154, t41161, t42859)
}

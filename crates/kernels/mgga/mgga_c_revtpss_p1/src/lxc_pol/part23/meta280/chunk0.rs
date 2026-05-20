//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1503/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1503<F: Float>(t10845: F, t2487: F, t2482: F, t27: F, t2719: F, t820: F, t843: F, t821: F, t235: F) -> (F, F, F, F, F, F) {
    let t10846 = t10845 * t2487;
    let t10850 = t2482 * t2719 * t27;
    let t10858 = t820 * t2719 * t843;
    let t10866 = t821 * t821;
    let t10867 = F::new(1.0) / t10866;
    let t10868 = t10867 * t235;
    (t10846, t10850, t10858, t10866, t10867, t10868)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1778/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1778<F: Float>(t10845: F, t2487: F, t2482: F, t27: F, t2719: F, t221: F, t2485: F, t2724: F, t2741: F, t2756: F, t820: F, t843: F) -> (F, F, F, F, F, F) {
    let t10846 = t10845 * t2487;
    let t10850 = t2482 * t2719 * t27;
    let t10852 = t2485 * t221 * t2724;
    let t10853 = t10850 * t10852;
    let t10855 = t2741 * t2756;
    let t10858 = t820 * t2719 * t843;
    (t10846, t10850, t10852, t10853, t10855, t10858)
}

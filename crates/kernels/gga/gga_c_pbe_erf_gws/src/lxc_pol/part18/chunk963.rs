//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 963/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk963<F: Float>(t10784: F, t7435: F, t587: F, t10788: F, t7699: F, t3445: F, t579: F, t3444: F, t582: F, t185: F, t1006: F, t2756: F) -> (F, F, F, F, F) {
    let t10861 = t7435 * t10784;
    let t10863 = F::new(32.0) / F::new(81.0) * t587 * t10861;
    let t10864 = t7699 * t10788;
    let t10866 = F::new(16.0) / F::new(27.0) * t587 * t10864;
    let t10870 = F::new(2.0) / F::new(15.0) * t579 * t3445;
    let t10871 = t582 * t3444;
    let t10872 = t185 * t10871;
    let t10873 = F::new(4.0) / F::new(45.0) * t10872;
    let t10874 = t1006 * t2756;
    (t10863, t10866, t10870, t10873, t10874)
}

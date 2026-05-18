//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 831/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk831<F: Float>(t2673: F, t4934: F, t639: F, t219: F, t5480: F, t2679: F, t2580: F, t5125: F, t1820: F, t2756: F, t579: F, t532: F) -> (F, F, F, F, F) {
    let t7874 = t4934 * t2673;
    let t7876 = F::new(32.0) / F::new(135.0) * t639 * t7874;
    let t7877 = t5480 * t219;
    let t7878 = t7877 * t2679;
    let t7880 = F::new(16.0) / F::new(81.0) * t639 * t7878;
    let t7888 = t5125 * t2580;
    let t7890 = F::new(32.0) / F::new(135.0) * t1820 * t7888;
    let t7905 = F::new(8.0) / F::new(45.0) * t579 * t2756;
    let t7906 = F::new(4.0) * t532;
    (t7876, t7880, t7890, t7905, t7906)
}

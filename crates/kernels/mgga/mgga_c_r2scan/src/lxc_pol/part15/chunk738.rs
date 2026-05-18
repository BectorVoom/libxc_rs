//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 738/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk738<F: Float>(t5942: F, t615: F, t1757: F, t1679: F, t584: F, t1685: F, t591: F, t1684: F, t2065: F, t595: F, t637: F, t2068: F) -> (F, F, F, F) {
    let t5943 = t615 * t5942;
    let t5945 = F::new(0.67745118933333333331e-2) * t1757 * t5943;
    let t5946 = t584 * t1679;
    let t5947 = t1685 * t591;
    let t5948 = t1684 * t5947;
    let t5950 = F::new(0.254044196e-2) * t5946 * t5948;
    let t5951 = t595 * t2065;
    let t5952 = t5951 * t637;
    let t5954 = t595 * t2068;
    (t5945, t5950, t5952, t5954)
}

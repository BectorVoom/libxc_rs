//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 659/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk659<F: Float>(t10734: F, t2958: F, t7068: F, t2580: F, t1897: F, t2549: F, t3444: F, t8528: F, t883: F, t2562: F, t943: F, t3437: F) -> (F, F, F, F, F, F) {
    let t10735 = F::cast_from(0.42725145723012357132e-3_f64) * t10734;
    let t10736 = t2958 * t7068;
    let t10737 = t2580 * t10736;
    let t10739 = F::cast_from(0.15381052460284448567e-1_f64) * t1897 * t10737;
    let t10740 = t2549 * t3444;
    let t10741 = F::cast_from(0.32043859292259267849e-3_f64) * t10740;
    let t10742 = t883 * t8528;
    let t10743 = t2562 * t10742;
    let t10744 = t943 * t10743;
    let t10745 = F::cast_from(0.32043859292259267849e-3_f64) * t10744;
    let t10746 = t2549 * t3437;
    (t10735, t10736, t10739, t10741, t10745, t10746)
}

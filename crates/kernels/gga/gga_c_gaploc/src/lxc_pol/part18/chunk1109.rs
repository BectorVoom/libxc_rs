//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1109/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1109<F: Float>(t10773: F, t7129: F, t1024: F, t2508: F, t7589: F, t10784: F, t1841: F, t1881: F, t2610: F, t32214: F, t32529: F, t32532: F, t32535: F, t32539: F, t32541: F, t32543: F, t32545: F, t32548: F, t32553: F, t32555: F, t3464: F, t5269: F, t5396: F) -> (F,) {
    let t32557 = 0.15381052460284448567e-1 * t7129 * t10773;
    let t32560 = 0.76905262301422242837e-2 * t2508 * t7589 * t1024;
    let t32565 = -t32529 + t32532 + t32535 + 0.30762104920568897134e-1 * t7129 * t10784 + t32539 + t32541 + t32543 + t32545 - t32548 + 0.15381052460284448567e-1 * t5269 * t3464 * t1881 + t32553 + t32555 + t32557 + t32560 + 0.51270174867614828558e-2 * t1841 * t5396 * t2610 * t32214;
    (t32565,)
}

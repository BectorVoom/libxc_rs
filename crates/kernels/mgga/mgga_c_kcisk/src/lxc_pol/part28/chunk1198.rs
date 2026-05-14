//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1198/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1198<F: Float>(t1908: F, t34374: F, t10024: F, t2041: F, t18179: F, t18925: F, t2049: F, t2666: F, t2815: F, t33153: F, t33306: F, t34287: F, t34290: F, t34293: F, t34294: F, t34296: F, t34299: F, t34308: F, t34309: F, t34312: F, t7656: F, t7659: F, t9763: F, t9772: F) -> (F, F, F) {
    let t34375 = t1908 * t34374;
    let t34377 = t10024 * t2041;
    let t34385 = -t18179 * t2815 + 2.0 * t18925 * t9763 - t2049 * t34377 - t2666 * t33306 + 2.0 * t33153 * t7659 - t7656 * t9772 - t34287 + t34290 + t34293 + t34294 - t34296 - t34299 - t34308 + t34309 - t34312;
    (t34375, t34377, t34385)
}

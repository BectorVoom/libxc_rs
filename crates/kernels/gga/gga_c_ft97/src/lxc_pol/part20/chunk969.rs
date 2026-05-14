//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 969/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk969<F: Float>(t237: F, t39: F, t240: F, t7513: F, t9681: F, t294: F, t7639: F, t10363: F, t1113: F, t230: F, t420: F, t1127: F, t213: F, t1208: F, t2: F, t7242: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t30815 = t237 * t39;
    let t33300 = 1.0 / t7513 / t240;
    let t33432 = t9681 * t39;
    let t33828 = 1.0 / t7639 / t294;
    let t33939 = t10363 * t39;
    let t35409 = t230 * t1113;
    let t35410 = t420 * t35409;
    let t35414 = t230 * t1127;
    let t35415 = t420 * t35414;
    let t35455 = t230 * t213;
    let t35456 = t420 * t35455;
    let t35870 = t230 * t1208;
    let t36452 = t7242 * t2;
    (t30815, t33300, t33432, t33828, t33939, t35410, t35415, t35455, t35456, t35870, t36452)
}

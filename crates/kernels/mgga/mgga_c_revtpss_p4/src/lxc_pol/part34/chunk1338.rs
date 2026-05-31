//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1338/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1338<F: Float>(t105819: F, t1518: F, t572: F, t28276: F, t5920: F, t2042: F, t25055: F, t6941: F, t7950: F, t1916: F, t30185: F, t30188: F) -> (F, F, F, F, F, F) {
    let t114850 = F::cast_from(18.0_f64) * t572 * t105819 * t1518;
    let t114853 = F::cast_from(18.0_f64) * t572 * t28276 * t5920;
    let t114865 = F::cast_from(3.0_f64) * t25055 * t2042;
    let t114871 = F::cast_from(18.0_f64) * t6941 * t7950;
    let t114873 = F::cast_from(18.0_f64) * t1916 * t30185;
    let t114875 = F::cast_from(36.0_f64) * t1916 * t30188;
    (t114850, t114853, t114865, t114871, t114873, t114875)
}

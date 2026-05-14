//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1102/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1102<F: Float>(t222: F, t20918: F, t20921: F, t20924: F, t20925: F, t20926: F, t20928: F, t20930: F, t20933: F, t20936: F, t20939: F, t21342: F, t22051: F, t22157: F, t240: F, t15823: F, t295: F, zeta_threshold: F) -> (F, F, F) {
    let t223 = t222 <= zeta_threshold;
    let t22160 = t20918 - t20921 + t20924 - t20925 - t20926 + t20928 - t20930 - t20933 + t20936 + t20939 - t21342 + t240 * (t22051 + t22157);
    let t22173 = piecewise3(t223, 0.0, -t15823);
    let t22174 = t295 * t22173;
    (t22160, t22173, t22174)
}

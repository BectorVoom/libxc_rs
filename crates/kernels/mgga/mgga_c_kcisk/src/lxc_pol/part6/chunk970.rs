//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 970/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk970<F: Float>(t30141: F, t2356: F, t9296: F, t10339: F, t10342: F, t10351: F, t12815: F, t30124: F, t30127: F, t30129: F, t30133: F, t30135: F, t30140: F) -> F {
    let t30142 = F::new(3.0) / F::new(8.0) * t30141;
    let t30143 = t2356 * t9296;
    let t30144 = F::new(3.0) / F::new(16.0) * t30143;
    let t30145 = -t30124 - t10339 + t10342 - t30127 - t30129 + t10351 + t30133 - t30135 - t30140 + t30142 - t12815 + t30144;
    t30145
}

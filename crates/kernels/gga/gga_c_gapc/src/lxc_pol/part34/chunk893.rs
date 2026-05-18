//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 893/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk893<F: Float>(t10158: F, t996: F, t3218: F, t1560: F, t315: F, t2160: F, t2165: F, t3244: F, t126: F, t2190: F, t284: F, t10137: F, t10140: F, t10144: F, t10148: F, t10151: F, t10154: F, t10156: F) -> (F, F, F, F, F) {
    let t10159 = t996 * t10158;
    let t10160 = t10159 * t3218;
    let t10162 = t1560 * t315;
    let t10163 = t2160 * t10162;
    let t10165 = t2165 * t3244;
    let t10167 = t126 * t2190;
    let t10168 = t284 * t10167;
    let t10170 = -F::new(0.56366309740899397906e-3) * t10137 - F::new(0.18788769913633132635e-4) * t10140 - F::new(0.56366309740899397906e-3) * t10144 + F::new(0.56366309740899397906e-3) * t10148 + F::new(0.56366309740899397906e-3) * t10151 + F::new(0.3556532540941297432e-4) * t10154 + F::new(0.3556532540941297432e-4) * t10156 - F::new(0.82073827867876094584e-5) * t10160 - F::new(0.11135477635479903275e-5) * t10163 - F::new(0.82200868372144955279e-5) * t10165 + F::new(0.28183154870449698953e-3) * t10168;
    (t10160, t10163, t10165, t10168, t10170)
}

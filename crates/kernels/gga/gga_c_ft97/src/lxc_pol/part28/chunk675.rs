//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 675/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk675<F: Float>(t32211: F, t44: F, t5551: F, t1302: F, t1291: F, t1295: F, t2035: F, t22623: F, t22687: F, t32181: F, t32185: F, t32187: F, t32190: F, t32208: F, t401: F, t5518: F, t5530: F, t5534: F, t5557: F, t5604: F, t7178: F, t7181: F, t7318: F, t7867: F, t79: F) -> (F, F, F, F) {
    let t32213 = 1.0 / t44 / t32211;
    let t32214 = t5551 * t32213;
    let t32215 = t32214 * t1302;
    let t32218 = t32181 + 0.20429954681481481482e0 * t7178 * t5604 - t32185 + 0.11854761295685025975e-1 * t79 * t32187 - 0.19762785756235085044e-4 * t7867 * t2035 * t32190 - 0.88910709717637694816e-2 * t5518 * t1291 - 0.88910709717637694816e-2 * t5534 * t1291 - 0.21080304806650757379e-3 * t1295 * t5557 + 0.47419045182740103902e-1 * t1295 * t5530 + 0.39525571512470170088e-4 * t22687 * t2035 * t7318 * t401 + 0.52700762016626893448e-4 * t7181 * t32208 + 0.78129887353338233165e-6 * t22623 * t32215;
    (t32213, t32214, t32215, t32218)
}

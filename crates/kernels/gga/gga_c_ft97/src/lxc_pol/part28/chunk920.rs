//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 920/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk920<F: Float>(t101161: F, t2035: F, t53: F, t136993: F, t137035: F, t145322: F, t145379: F, t22842: F, t25726: F, t25730: F, t25816: F, t32295: F, t34427: F, t36368: F, t38176: F, t45572: F, t7853: F, t7857: F, t7867: F, t92399: F, t92489: F) -> (F,) {
    let t145556 = t2035 * t101161 * t53;
    let t145580 = -0.20869152414369355073e-1 * t136993 - 0.52700762016626893448e-4 * t36368 * t145556 + 0.26350381008313446725e-3 * t7867 * t145556 + 0.88910709717637694816e-2 * t22842 * t7853 * t25726 - 0.47419045182740103902e-1 * t22842 * t7853 * t25816 + 0.21080304806650757379e-3 * t22842 * t38176 * t25730 + 0.88910709717637694816e-2 * t92399 * t34427 + 0.88910709717637694816e-2 * t92489 * t34427 - 0.25537443351851851852e-1 * t137035 - 0.25845121844514357744e-4 * t32295 * t145379 + 0.60102574844279699039e-6 * t7857 * t45572 * t145322;
    (t145580,)
}

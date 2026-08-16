//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1055/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1055(t101161: f64, t2035: f64, t53: f64, t136993: f64, t137035: f64, t145322: f64, t145379: f64, t22842: f64, t25726: f64, t25730: f64, t25816: f64, t32295: f64, t34427: f64, t36368: f64, t38176: f64, t45572: f64, t7853: f64, t7857: f64, t7867: f64, t92399: f64, t92489: f64) -> f64 {
    let t145556 = t2035 * t101161 * t53;
    let t145580 = -0.20869152414369355073e-1_f64 * t136993 - 0.52700762016626893448e-4_f64 * t36368 * t145556 + 0.26350381008313446725e-3_f64 * t7867 * t145556 + 0.88910709717637694816e-2_f64 * t22842 * t7853 * t25726 - 0.47419045182740103902e-1_f64 * t22842 * t7853 * t25816 + 0.21080304806650757379e-3_f64 * t22842 * t38176 * t25730 + 0.88910709717637694816e-2_f64 * t92399 * t34427 + 0.88910709717637694816e-2_f64 * t92489 * t34427 - 0.25537443351851851852e-1_f64 * t137035 - 0.25845121844514357744e-4_f64 * t32295 * t145379 + 0.60102574844279699039e-6_f64 * t7857 * t45572 * t145322;
    t145580
}

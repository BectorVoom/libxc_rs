//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1326/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1326(t10044: f64, t10047: f64, t10054: f64, t10083: f64, t10085: f64, t10089: f64, t10092: f64, t10094: f64, t10189: f64, t11404: f64, t11445: f64, t11456: f64, t18661: f64, t2380: f64, t2381: f64, t27104: f64, t28128: f64, t28166: f64, t28174: f64, t3185: f64, t3186: f64, t3206: f64, t3913: f64, t3919: f64, t394: f64, t406: f64, t8380: f64, t919: f64, t921: f64) -> f64 {
    let t32137 = -0.12862205435420921092e-2_f64 * t2380 * t2381 * t3919 * t10054 + 0.12862205435420921092e-2_f64 * t3185 * t406 * t27104 * t11404 + 0.12862205435420921092e-2_f64 * t3185 * t406 * t10083 * t10092 + 0.12862205435420921092e-2_f64 * t3206 * t2381 * t3913 * t10054 - 0.64311027177104605458e-3_f64 * t3206 * t406 * t8380 * t11456 - 0.64311027177104605458e-3_f64 * t3206 * t406 * t3186 * t394 * t10189 + 0.34299214494455789577e-2_f64 * t10047 * t10085 - 0.51448821741683684368e-2_f64 * t2380 * t18661 * t11445 * t919 * t921 + 0.13719685797782315831e-1_f64 * t10044 * t10089 - 0.13719685797782315831e-1_f64 * t10044 * t10094 + 0.14291339372689912324e-3_f64 * t28128 - t28166 / 72.0_f64 + t28174 / 144.0_f64;
    t32137
}

//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1323/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1323(t10044: f64, t10047: f64, t1220: f64, t18979: f64, t19026: f64, t19030: f64, t2185: f64, t22938: f64, t23017: f64, t23093: f64, t23104: f64, t23122: f64, t23163: f64, t23236: f64, t23275: f64, t23333: f64, t23338: f64, t23341: f64, t23345: f64, t23355: f64, t23362: f64, t23390: f64, t2371: f64, t2380: f64, t2381: f64, t2382: f64, t2387: f64, t2396: f64, t2888: f64, t3026: f64, t3174: f64, t3175: f64, t3185: f64, t3206: f64, t3214: f64, t6106: f64, t6419: f64, t6438: f64, t6472: f64, t6509: f64, t7945: f64, t824: f64, t8254: f64, t8276: f64, t8450: f64, t919: f64, t921: f64, t931: f64) -> f64 {
    let t23394 = -0.68598428988911579154e-2_f64 * t10044 * t6419 + 0.34299214494455789577e-2_f64 * t10047 * t6472 + t1220 * t6438 / 6.0_f64 - 0.11433071498151929859e-2_f64 * t3214 * t6509 - 0.33875767401931644027e-3_f64 * t23355 + 0.91464571985215438872e-2_f64 * t23362 + t23093 + t23341 + t23236 + 0.63517063878621832551e-4_f64 * t23345 + t23017 + t23390 + t23275 + t23163 - 5.0_f64 / 1296.0_f64 * t23338 + 0.42874018118069736972e-3_f64 * t23122 + t23333 + t22938 - 0.12862205435420921092e-2_f64 * t2380 * t2381 * t7945 * t919 * t921 - 0.12862205435420921092e-2_f64 * t2380 * t2381 * t3026 * t2387 * t921 + t3174 * t2888 * t931 * t7945 * t824 / 16.0_f64 - 5.0_f64 / 432.0_f64 * t19026 + t19030 / 144.0_f64 + t3174 * t2888 * t8276 * t2185 / 16.0_f64 + t3174 * t2888 * t3175 * t6106 / 48.0_f64 - 0.12862205435420921092e-2_f64 * t8450 * t8254 * t18979 * t2382 - 0.25724410870841842183e-2_f64 * t3185 * t2381 * t23104 * t2371 + 0.12862205435420921092e-2_f64 * t3206 * t2381 * t23104 * t2396;
    t23394
}

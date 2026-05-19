//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1323/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1323<F: Float>(t10044: F, t10047: F, t1220: F, t18979: F, t19026: F, t19030: F, t2185: F, t22938: F, t23017: F, t23093: F, t23104: F, t23122: F, t23163: F, t23236: F, t23275: F, t23333: F, t23338: F, t23341: F, t23345: F, t23355: F, t23362: F, t23390: F, t2371: F, t2380: F, t2381: F, t2382: F, t2387: F, t2396: F, t2888: F, t3026: F, t3174: F, t3175: F, t3185: F, t3206: F, t3214: F, t6106: F, t6419: F, t6438: F, t6472: F, t6509: F, t7945: F, t824: F, t8254: F, t8276: F, t8450: F, t919: F, t921: F, t931: F) -> F {
    let t23394 = -F::cast_from(0.68598428988911579154e-2_f64) * t10044 * t6419 + F::cast_from(0.34299214494455789577e-2_f64) * t10047 * t6472 + t1220 * t6438 / F::new(6.0) - F::cast_from(0.11433071498151929859e-2_f64) * t3214 * t6509 - F::cast_from(0.33875767401931644027e-3_f64) * t23355 + F::cast_from(0.91464571985215438872e-2_f64) * t23362 + t23093 + t23341 + t23236 + F::cast_from(0.63517063878621832551e-4_f64) * t23345 + t23017 + t23390 + t23275 + t23163 - F::new(5.0) / F::new(1296.0) * t23338 + F::cast_from(0.42874018118069736972e-3_f64) * t23122 + t23333 + t22938 - F::cast_from(0.12862205435420921092e-2_f64) * t2380 * t2381 * t7945 * t919 * t921 - F::cast_from(0.12862205435420921092e-2_f64) * t2380 * t2381 * t3026 * t2387 * t921 + t3174 * t2888 * t931 * t7945 * t824 / F::new(16.0) - F::new(5.0) / F::new(432.0) * t19026 + t19030 / F::new(144.0) + t3174 * t2888 * t8276 * t2185 / F::new(16.0) + t3174 * t2888 * t3175 * t6106 / F::new(48.0) - F::cast_from(0.12862205435420921092e-2_f64) * t8450 * t8254 * t18979 * t2382 - F::cast_from(0.25724410870841842183e-2_f64) * t3185 * t2381 * t23104 * t2371 + F::cast_from(0.12862205435420921092e-2_f64) * t3206 * t2381 * t23104 * t2396;
    t23394
}

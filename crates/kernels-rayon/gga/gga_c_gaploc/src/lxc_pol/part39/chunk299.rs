//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 299/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk299(t1628: f64, t917: f64, t1424: f64, t1429: f64, t1450: f64, t1456: f64, t1562: f64, t1572: f64, t1641: f64, t1646: f64, t2362: f64, t2369: f64, t2372: f64, t2375: f64, t2379: f64, t2382: f64, t2385: f64, t2386: f64, t2390: f64, t2392: f64, t2395: f64, t2399: f64, t2402: f64, t2407: f64, t2411: f64, t2413: f64, t2418: f64, t536: f64, t567: f64, t574: f64, t597: f64, t908: f64) -> f64 {
    let t2421 = t1628 * t917;
    let t2426 = 0.12780975317973583226e0_f64 * t2362 - 0.14896037479937677779e-1_f64 * t2369 - 0.39722766613167140743e-1_f64 * t2372 * t1424 + 0.39722766613167140743e-1_f64 * t1429 * t2375 - 0.46011511144704899612e1_f64 * t574 * t2379 + 0.11502877786176224903e2_f64 * t597 * t2382 - 0.10725146985555128001e1_f64 * t2385 * t2386 - 0.29792074959875355558e-1_f64 * t2390 + 0.71500979903700853338e0_f64 * t1572 * t2392 + 0.23005755572352449806e1_f64 * t567 * t2395 - 0.23005755572352449806e1_f64 * t1450 * t2399 - 0.35750489951850426669e0_f64 * t2402 * t1646 + 0.35750489951850426669e0_f64 * t536 * t2407 + 0.14896037479937677779e-1_f64 * t2411 + 0.35750489951850426669e0_f64 * t1456 * t2413 - 0.69017266717057349418e1_f64 * t1562 * t2418 + 0.30674340763136599741e1_f64 * t597 * t2421 - 0.23005755572352449806e1_f64 * t1641 * t908;
    t2426
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 461/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk461<F: Float>(t1564: F, t874: F, t475: F, t1445: F, t1628: F, t917: F, t1424: F, t1429: F, t1450: F, t1456: F, t1562: F, t1572: F, t1641: F, t1646: F, t2362: F, t2369: F, t2372: F, t2375: F, t2379: F, t2382: F, t2385: F, t2386: F, t2390: F, t2392: F, t2395: F, t2399: F, t2402: F, t2407: F, t2411: F, t2413: F, t536: F, t567: F, t574: F, t597: F, t908: F) -> (F, F, F, F, F) {
    let t2416 = t1564 * t874;
    let t2417 = t2416 * t475;
    let t2418 = t1445 * t2417;
    let t2421 = t1628 * t917;
    let t2426 = 0.12780975317973583226e0 * t2362 - 0.14896037479937677779e-1 * t2369 - 0.39722766613167140743e-1 * t2372 * t1424 + 0.39722766613167140743e-1 * t1429 * t2375 - 0.46011511144704899612e1 * t574 * t2379 + 0.11502877786176224903e2 * t597 * t2382 - 0.10725146985555128001e1 * t2385 * t2386 - 0.29792074959875355558e-1 * t2390 + 0.71500979903700853338e0 * t1572 * t2392 + 0.23005755572352449806e1 * t567 * t2395 - 0.23005755572352449806e1 * t1450 * t2399 - 0.35750489951850426669e0 * t2402 * t1646 + 0.35750489951850426669e0 * t536 * t2407 + 0.14896037479937677779e-1 * t2411 + 0.35750489951850426669e0 * t1456 * t2413 - 0.69017266717057349418e1 * t1562 * t2418 + 0.30674340763136599741e1 * t597 * t2421 - 0.23005755572352449806e1 * t1641 * t908;
    (t2416, t2417, t2418, t2421, t2426)
}

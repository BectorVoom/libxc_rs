//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 840/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk840<F: Float>(t11451: F, t5126: F, t11450: F, t1936: F, t5462: F, t144: F, t1453: F, t5526: F, t674: F, t11373: F, t11375: F, t11377: F, t11382: F, t11385: F, t11389: F, t11392: F, t11403: F, t11406: F, t11410: F, t11415: F, t11421: F, t11426: F, t11432: F, t11436: F, t11440: F, t11443: F, t11445: F) -> (F, F, F, F) {
    let t11452 = t11451 * t5126;
    let t11453 = t11450 * t11452;
    let t11455 = t5462 * t1936;
    let t11456 = t1453 * t144;
    let t11458 = t11456 * t674 * t5526;
    let t11459 = t11455 * t11458;
    let t11461 = -0.45289771048911752714e-7 * t11373 + 0.10551620497652752682e-7 * t11375 + 0.10551620497652752682e-7 * t11377 - 0.22099262292595577331e-8 * t11382 - 0.33148893438893365995e-7 * t11385 + 0.24761136101158459627e-5 * t11389 + 0.28985453471303521737e-5 * t11392 - 0.71700964683956570107e-9 * t11403 + 0.20241536458333333334e-4 * t11406 + 0.4216986762152777778e-6 * t11410 - 0.36898634168836805558e-6 * t11415 - 0.23671453668231209419e-4 * t11421 - 0.10120768229166666667e-3 * t11426 + 0.15387779892410264328e-8 * t11432 - 0.10860115658064651693e-4 * t11436 - 0.10860115658064651693e-4 * t11440 - 0.28431716307092827285e-6 * t11443 - 0.32043930324263587129e-6 * t11445 + 0.49166375783284505216e-8 * t11453 + 0.50551591594011046914e-6 * t11459;
    (t11452, t11455, t11458, t11461)
}

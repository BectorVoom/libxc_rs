//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 933/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk933<F: Float>(t11451: F, t5126: F, t11450: F, t1936: F, t5462: F, t144: F, t1453: F, t5526: F, t674: F, t11373: F, t11375: F, t11377: F, t11382: F, t11385: F, t11389: F, t11392: F, t11403: F, t11406: F, t11410: F, t11415: F, t11421: F, t11426: F, t11432: F, t11436: F, t11440: F, t11443: F, t11445: F) -> (F, F, F, F) {
    let t11452 = t11451 * t5126;
    let t11453 = t11450 * t11452;
    let t11455 = t5462 * t1936;
    let t11456 = t1453 * t144;
    let t11458 = t11456 * t674 * t5526;
    let t11459 = t11455 * t11458;
    let t11461 = -F::cast_from(0.45289771048911752714e-7_f64) * t11373 + F::cast_from(0.10551620497652752682e-7_f64) * t11375 + F::cast_from(0.10551620497652752682e-7_f64) * t11377 - F::cast_from(0.22099262292595577331e-8_f64) * t11382 - F::cast_from(0.33148893438893365995e-7_f64) * t11385 + F::cast_from(0.24761136101158459627e-5_f64) * t11389 + F::cast_from(0.28985453471303521737e-5_f64) * t11392 - F::cast_from(0.71700964683956570107e-9_f64) * t11403 + F::cast_from(0.20241536458333333334e-4_f64) * t11406 + F::cast_from(0.4216986762152777778e-6_f64) * t11410 - F::cast_from(0.36898634168836805558e-6_f64) * t11415 - F::cast_from(0.23671453668231209419e-4_f64) * t11421 - F::cast_from(0.10120768229166666667e-3_f64) * t11426 + F::cast_from(0.15387779892410264328e-8_f64) * t11432 - F::cast_from(0.10860115658064651693e-4_f64) * t11436 - F::cast_from(0.10860115658064651693e-4_f64) * t11440 - F::cast_from(0.28431716307092827285e-6_f64) * t11443 - F::cast_from(0.32043930324263587129e-6_f64) * t11445 + F::cast_from(0.49166375783284505216e-8_f64) * t11453 + F::cast_from(0.50551591594011046914e-6_f64) * t11459;
    (t11452, t11455, t11458, t11461)
}

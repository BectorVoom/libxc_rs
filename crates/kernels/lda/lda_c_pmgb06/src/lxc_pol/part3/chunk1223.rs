//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1223/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1223<F: Float>(t10670: F, t10790: F, t10792: F, t117: F, t123: F, t125: F, t14222: F, t14275: F, t14279: F, t14284: F, t14287: F, t14291: F, t14293: F, t14298: F, t14300: F, t14303: F, t14306: F, t14308: F, t14318: F, t14319: F, t14321: F, t14322: F, t14325: F, t14326: F, t14330: F, t14331: F, t14335: F, t14336: F, t14338: F, t14339: F, t14342: F, t14343: F, t14345: F, t14361: F, t14366: F, t14367: F, t14369: F, t14370: F, t14373: F, t14378: F, t14380: F, t14381: F, t14385: F, t14386: F, t14388: F, t14389: F, t14392: F, t14393: F, t14395: F, t14396: F, t14402: F, t14403: F, t14405: F, t14406: F, t14409: F, t14410: F, t14412: F, t14413: F, t14417: F, t14418: F, t14420: F, t14421: F, t14424: F, t14428: F, t14430: F, t14431: F, t14436: F, t14437: F, t14439: F, t14440: F, t14443: F, t14444: F, t14453: F, t14454: F, t14458: F, t14459: F, t14461: F, t14462: F, t14473: F, t14486: F) -> F {
    let t14497 = F::cast_from(0.2634331482256014_f64) * t10670 - F::cast_from(0.025899545097903542_f64) * t14275 + F::cast_from(0.0034679929861433484_f64) * t14279 - t14284 - F::cast_from(0.005926167098672845_f64) * t14287 - t14291 - F::cast_from(0.005926167098672845_f64) * t14293 + t14298 + F::cast_from(0.01975389032890948_f64) * t14300 - F::cast_from(0.0014862827083471494_f64) * t14303 - F::cast_from(0.07184540406152766_f64) * t10790 + F::cast_from(0.1890324433388467_f64) * t14306 - F::cast_from(0.00011865309871651405_f64) * t14308 - F::cast_from(0.005388405304614574_f64) * t123 * t125 * (t14486 + t14473 + t14459 + t14461 + t14462 + t14458 + t14453 + t14454 + t14443 + t14444 + t14439 + t14440 + t14436 + t14437 + t14430 + t14431 + t14428 + t14421 + t14424 + t14417 + t14418 + t14420 + t14412 + t14413 + t14409 + t14410 + t14402 + t14403 + t14405 + t14406 + t14395 + t14396 + t14389 + t14392 + t14393 + t14385 + t14386 + t14388 + t14380 + t14381 + t14378 + t14373 + t14366 + t14367 + t14369 + t14370 + t14361 + t14343 + t14345 + t14339 + t14342 + t14335 + t14336 + t14338 + t14330 + t14331 + t14325 + t14326 + t14321 + t14322 + t14318 + t14319 + t14222) * t117 - t10792;
    t14497
}

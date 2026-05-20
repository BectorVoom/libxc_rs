//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2806/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2806<F: Float>(t23414: F, t689: F, t779: F, t23413: F, t41070: F, t686: F, t72: F, t18805: F, t50208: F, t2765: F, t39550: F, t39554: F, t39557: F, t39558: F, t50184: F, t50187: F, t50205: F, t61355: F, t61361: F, t61367: F, t61371: F, t61378: F) -> F {
    let t75974 = t689 * t779 * t23414;
    let t75978 = t41070 * t23413 * t72 * t686;
    let t75984 = t50208 * t18805;
    let t75990 = -F::cast_from(0.11044544084478153697e-3_f64) * t39550 + F::cast_from(0.39029762157531132076e-1_f64) * t61355 + F::cast_from(0.32927245914677557992e-1_f64) * t75974 - t50184 + t50187 + t39554 - F::cast_from(0.58544643236296698112e-1_f64) * t75978 + t39557 - F::cast_from(0.46263278077393568556e-2_f64) * t39558 - F::cast_from(0.21951497276451705328e-1_f64) * t61361 + F::cast_from(0.43902994552903410656e-1_f64) * t61367 + F::cast_from(0.34697458558045176418e-2_f64) * t61371 + F::cast_from(0.58544643236296698112e-1_f64) * t75984 - F::cast_from(0.39512695097613069591e1_f64) * t2765 * t23414 + F::cast_from(0.16463622957338778996e-1_f64) * t61378 - F::cast_from(0.91069445034239308177e-1_f64) * t50205;
    t75990
}

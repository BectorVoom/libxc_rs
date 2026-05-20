//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3237/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3237<F: Float>(t18811: F, t2435: F, t18825: F, t2453: F, t2458: F, t6042: F, t18785: F, t689: F, t779: F, t18316: F, t887: F, t14979: F, t15029: F, t39554: F, t39557: F, t39558: F, t39562: F, t39565: F, t39567: F, t39573: F, t4474: F, t50161: F, t50186: F, t50198: F, t50201: F, t50205: F, t50209: F, t50240: F) -> F {
    let t61361 = t2435 * t18811;
    let t61367 = t2435 * t18825;
    let t61371 = t2453 * t6042 * t2458;
    let t61378 = t689 * t779 * t18785;
    let t61385 = t689 * t18316 * t887;
    let t61387 = F::cast_from(0.52039682876708176102e-1_f64) * t50186 + t39554 + t39557 - F::cast_from(0.92526556154787137112e-2_f64) * t39558 - F::cast_from(0.73171657588172351096e-2_f64) * t61361 - F::cast_from(0.65049603595885220126e-3_f64) * t39562 + F::cast_from(0.13009920719177044025e-2_f64) * t39565 - F::cast_from(0.13009920719177044025e-1_f64) * t39567 - F::cast_from(0.13009920719177044025e-2_f64) * t39573 + F::cast_from(0.14634331517634470219e-1_f64) * t61367 + F::cast_from(0.10975748638225852664e-1_f64) * t50198 + F::cast_from(0.11565819519348392139e-2_f64) * t61371 + F::cast_from(0.19514881078765566038e-1_f64) * t50201 - F::cast_from(0.15805078039045227836e2_f64) * t50240 * t50161 * t15029 + F::cast_from(0.10975748638225852664e-1_f64) * t61378 - F::cast_from(0.13170898365871023197e1_f64) * t4474 * t14979 - F::cast_from(0.60712963356159538786e-1_f64) * t50205 + F::cast_from(0.39029762157531132076e-1_f64) * t50209 + F::cast_from(0.10975748638225852664e-1_f64) * t61385;
    t61387
}

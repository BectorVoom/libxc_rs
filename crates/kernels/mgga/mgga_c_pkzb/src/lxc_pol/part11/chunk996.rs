//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 996/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk996<F: Float>(t11817: F, t204: F, t334: F, t1731: F, t218: F, t344: F, t5555: F, t847: F, t16194: F, t339: F, t930: F, t336: F, t2316: F, t2319: F, t2294: F, t2238: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t18439 = t204 * t11817 * t334;
    let t18440 = 0.31310740740740740741e1 * t18439;
    let t18442 = t218 * t1731 * t344;
    let t18443 = 0.13490888888888888889e1 * t18442;
    let t18445 = t218 * t5555 * t847;
    let t18468 = 280.0 / 81.0 * t18439;
    let t18480 = 1.0 / t339 / t16194 / t930 / 96.0;
    let t18492 = f64::powf(t336, -0.25e1);
    let t18508 = t2316 * t2316;
    let t18509 = 1.0 / t18508;
    let t18512 = t2319 * t2319;
    let t18513 = 1.0 / t18512;
    let t18520 = 1.0 / t2316 / t2294;
    let t18554 = 0.31003950617283950618e1 * t18439;
    let t18555 = 0.13388493827160493828e1 * t18442;
    let t18587 = t2238 * t2238;
    (t18439, t18440, t18442, t18443, t18445, t18468, t18480, t18492, t18509, t18513, t18520, t18554, t18555, t18587)
}

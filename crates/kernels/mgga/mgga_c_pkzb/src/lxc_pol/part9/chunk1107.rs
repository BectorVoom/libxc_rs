//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1107/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1107<F: Float>(t218: F, t6189: F, t675: F, t18439: F, t16194: F, t339: F, t930: F, t336: F, t2316: F, t2319: F, t2294: F, t18442: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18457 = t218 * t675 * t6189;
    let t18468 = F::new(280.0) / F::new(81.0) * t18439;
    let t18480 = F::new(1.0) / t339 / t16194 / t930 / F::new(96.0);
    let t18492 = f64::powf(t336, -F::new(0.25e1));
    let t18508 = t2316 * t2316;
    let t18509 = F::new(1.0) / t18508;
    let t18512 = t2319 * t2319;
    let t18513 = F::new(1.0) / t18512;
    let t18520 = F::new(1.0) / t2316 / t2294;
    let t18554 = F::new(0.31003950617283950618e1) * t18439;
    let t18555 = F::new(0.13388493827160493828e1) * t18442;
    (t18457, t18468, t18480, t18492, t18509, t18513, t18520, t18554, t18555)
}

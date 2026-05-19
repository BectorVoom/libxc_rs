//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 730/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk730<F: Float>(t2704: F, t2718: F, t4518: F, t4521: F, t4524: F, t4529: F, t4531: F, t4533: F, t404: F, t389: F, t4510: F, t1291: F) -> (F, F) {
    let t4536 = -F::new(0.25319e1) * t4518 + F::cast_from(0.16879333333333333333e1_f64) * t4521 - F::cast_from(0.19692555555555555555e1_f64) * t4524 - F::cast_from(0.93011851851851851854e0_f64) * t2704 + F::cast_from(0.13651666666666666667e0_f64) * t4529 - F::cast_from(0.27303333333333333333e0_f64) * t4531 - F::cast_from(0.3185388888888888889e0_f64) * t4533 - F::cast_from(0.36514074074074074075e0_f64) * t2718;
    let t4537 = t4536 * t404;
    let t4538 = t389 * t4537;
    let t4539 = F::new(1.0) * t4538;
    let t4540 = t4510 * t404;
    let t4541 = t1291 * t4540;
    (t4539, t4541)
}

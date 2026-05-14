//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 812/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk812<F: Float>(t1885: F, t7645: F, t587: F, t1010: F, t5304: F, t1022: F, t1697: F, t1413: F, t1809: F, t1620: F, t1821: F, t7359: F, t1000: F, t1804: F, t5548: F, t2688: F, t5129: F) -> (F, F, F, F, F, F) {
    let t7646 = t1885 * t7645;
    let t7648 = 4.0 / 15.0 * t587 * t7646;
    let t7650 = 8.0 / 45.0 * t5304 * t1010;
    let t7651 = t1022 * t1697;
    let t7652 = t7651 * t1413;
    let t7653 = t1809 * t7652;
    let t7655 = 16.0 / 45.0 * t1620 * t7653;
    let t7656 = t1821 * t7359;
    let t7658 = 8.0 / 15.0 * t587 * t7656;
    let t7659 = t1000 * t1804;
    let t7660 = t5548 * t7659;
    let t7662 = 8.0 / 45.0 * t587 * t7660;
    let t7663 = t5129 * t2688;
    (t7648, t7650, t7655, t7658, t7662, t7663)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 794/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk794<F: Float>(t16531: F, t188: F, t1804: F, t185: F, t186: F, t1: F, t3: F, t4562: F, t672: F, t2000: F, t5935: F, t2007: F, t1996: F, t5927: F, t16515: F, t16520: F, t16522: F, t16525: F, t16527: F, t16529: F) -> (F, F) {
    let t16532 = t188 * t16531;
    let t16533 = t1804 * t1804;
    let t16537 = 16.0 / 5.0 * t185 * t186 * t16532 * t16533;
    let t16540 = t4562 * t1 * t3 * t672;
    let t16542 = t5935 * t2000;
    let t16544 = t5935 * t2007;
    let t16546 = t1996 * t5927;
    let t16548 = -t16515 - t16520 - t16522 + t16525 + t16527 + t16529 + t16537 + 0.43284165449459373508e0 * t16540 + 0.12985249634837812052e1 * t16542 + 0.43284165449459373508e0 * t16544 + 0.12985249634837812052e1 * t16546;
    (t16537, t16548)
}

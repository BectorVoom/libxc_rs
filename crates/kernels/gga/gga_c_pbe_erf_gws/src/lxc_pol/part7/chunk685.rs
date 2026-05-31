//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 685/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk685<F: Float>(t1733: F, t649: F, t661: F, t1621: F, t1620: F, t1622: F, t1724: F, t1664: F, t4352: F, t590: F, t587: F, t1673: F, t579: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5500 = t649 * t1733;
    let t5501 = t5500 * t661;
    let t5502 = t1621 * t5501;
    let t5504 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1620 * t5502;
    let t5505 = t1622 * t1724;
    let t5506 = t1621 * t5505;
    let t5508 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1620 * t5506;
    let t5509 = t1664 * t4352;
    let t5510 = t590 * t5509;
    let t5512 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t587 * t5510;
    let t5513 = t579 * t1673;
    (t5500, t5501, t5502, t5504, t5505, t5506, t5508, t5509, t5510, t5512, t5513)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 943/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk943<F: Float>(t1355: F, t1383: F, t169: F, t4598: F, t770: F, t413: F, t745: F, t16447: F, t242: F, t5697: F, t700: F, t18022: F, t5701: F, t1339: F, t1452: F, t39: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19023 = t169 * t1355 * t1383;
    let t19026 = t169 * t770 * t4598;
    let t19028 = t413 * t745;
    let t19031 = t169 * t16447 * t242;
    let t19035 = 0.20752137690161369243e1 * t169 * t5697 * t700;
    let t19037 = t169 * t18022 * t242;
    let t19040 = t169 * t5701 * t700;
    let t19044 = 0.84895108732478328721e0 * t169 * t1339 * t1383;
    let t19045 = t39 * t1452;
    (t19023, t19026, t19028, t19031, t19035, t19037, t19040, t19044, t19045)
}

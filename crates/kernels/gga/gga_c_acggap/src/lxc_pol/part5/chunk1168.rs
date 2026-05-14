//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1168/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1168<F: Float>(t13364: F, t17173: F, t322: F, t5615: F, t8790: F, t1026: F, t1165: F, t1173: F, t1181: F, t1203: F, t13286: F, t13287: F, t13671: F, t1531: F, t1567: F, t17139: F, t1859: F, t18788: F, t18805: F, t18808: F, t18810: F, t18812: F, t1884: F, t23234: F, t3176: F, t3196: F, t3266: F, t386: F, t418: F, t5616: F, t5679: F) -> (F,) {
    let t24246 = t17173 * t13364 * t8790 * t5615 * t322;
    let t24277 = -0.34299214494455789578e-2 * t24246 + 0.85748036236139473944e-2 * t418 * t1026 * t3266 * t1884 - 0.85748036236139473944e-3 * t418 * t386 * t5679 * t1203 + 0.13719685797782315831e-1 * t18788 + 0.25724410870841842183e-2 * t1531 * t1165 * t23234 * t13671 + 0.34299214494455789578e-2 * t18805 - 0.34299214494455789578e-1 * t17139 * t13364 * t1859 * t3196 - 0.13719685797782315831e-1 * t13286 * t13287 * t1859 * t3176 - 0.68598428988911579156e-2 * t18808 + 0.34299214494455789577e-2 * t18810 - 0.16006300097412701803e-1 * t18812 + 0.68598428988911579156e-2 * t1173 * t1181 * t1567 * t5616;
    (t24277,)
}

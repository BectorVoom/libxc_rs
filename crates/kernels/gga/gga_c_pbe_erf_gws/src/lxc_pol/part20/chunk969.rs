//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 969/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk969<F: Float>(t11602: F, t11604: F, t11613: F, t11632: F, t11646: F, t11650: F, t11665: F, t11670: F, t11695: F, t11699: F, t8901: F, t8927: F, t8960: F, t11762: F, t11766: F, t11768: F, t11770: F, t11772: F, t11775: F, t11780: F, t11784: F, t11789: F, t11796: F, t8969: F, t8971: F, t8973: F) -> (F, F) {
    let t12152 = -t11602 - t8901 - t11604 + t11613 - t11632 - t8927 + t11646 - t11650 + t11665 - t11670 + t8960 - t11695 + t11699;
    let t12153 = -t8969 + t8971 + t8973 - t11762 + t11766 - t11768 + t11770 + t11772 - t11775 - t11780 - t11784 + t11789 - t11796;
    (t12152, t12153)
}

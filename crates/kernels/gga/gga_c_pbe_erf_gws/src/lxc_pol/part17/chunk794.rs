//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 794/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk794<F: Float>(t551: F, t553: F, t6016: F, t1371: F, t1960: F, t1464: F, t285: F, t545: F, t159: F, t5984: F, t169: F, t274: F, t301: F, t922: F) -> (F, F, F, F, F) {
    let t6018 = t6016 * t551 * t553;
    let t6021 = t1960 * t1371 * t553;
    let t6028 = F::new(0.40679438125041687114e-2) * t1464 * t545 * t285;
    let t6032 = F::new(0.67153358174671991426e-2) * t5984 * t159 * t285;
    let t6036 = F::new(0.92478548207158653218e0) * t169 * t922 * t274 * t301;
    (t6018, t6021, t6028, t6032, t6036)
}

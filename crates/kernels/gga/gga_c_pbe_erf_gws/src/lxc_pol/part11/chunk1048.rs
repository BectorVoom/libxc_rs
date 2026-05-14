//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1048/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1048<F: Float>(t11275: F, t12399: F, t12412: F, t12413: F, t125: F, t18106: F, t18108: F, t18122: F, t18126: F, t26012: F, t26085: F, t26101: F, t26118: F, t26129: F, t26135: F, t2857: F, t33691: F, t33770: F, t33778: F, t33854: F, t34302: F, t3671: F, t3686: F, t42304: F, t48609: F, t48706: F, t5651: F, t8497: F, t967: F, t981: F) -> (F,) {
    let t48709 = -0.3486808982146430324e-2 * t33691 + t18106 - t18108 + 0.65586876954174354395e-3 * t26012 - 24.0 * t8497 * t34302 * t12412 - 0.18276876377896586758e-4 * t26085 - 0.47896936041018436376e-1 * t26101 - 24.0 * t33854 * t12413 - 3.0 * t3686 * t12399 - 0.31931290694012290916e0 * t33770 + t18122 + t18126 + 6.0 * t11275 * t5651 * t981 * t3671 + 0.11890099055206112556e1 * t33778 + 0.78054266140918933351e0 * t26118 - 0.63862581388024581833e0 * t26129 - 0.21862292318058118132e-3 * t26135 + 24.0 * t2857 * t42304 * t967 + (t48609 + t48706) * t125;
    (t48709,)
}

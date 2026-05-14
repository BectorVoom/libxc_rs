//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 829/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk829<F: Float>(t426: F, t49921: F, t12116: F, t14272: F, t14277: F, t2268: F, t2756: F, t3701: F, t44530: F, t44534: F, t44538: F, t44542: F, t44544: F, t44549: F, t44552: F, t44553: F, t44556: F, t44559: F, t44572: F, t44574: F, t44576: F, t535: F, t6305: F, t6313: F, t988: F) -> (F,) {
    let t49944 = t49921 * t426;
    let t49958 = 0.56910013271352299198e-1 * t2268 * t535 * t49944 - t44530 + t44534 - t44538 + t44542 - t44544 - t44549 + t44552 + t44553 + 0.15176003539027279787e0 * t6313 * t14272 + 0.56910013271352299198e-1 * t6305 * t14277 + 0.56910013271352299198e-1 * t2268 * t12116 * t988 + 0.56910013271352299198e-1 * t2268 * t3701 * t2756 + t44556 + t44559 + t44572 - t44574 + t44576;
    (t49958,)
}

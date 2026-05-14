//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 823/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk823<F: Float>(t40896: F, t2508: F, t28024: F, t2936: F, t13172: F, t1901: F, t43243: F, t43244: F, t43248: F, t43254: F, t43257: F, t43260: F, t43263: F, t43265: F, t43267: F, t43269: F, t43270: F, t43274: F, t43275: F, t43278: F, t43282: F, t779: F) -> (F,) {
    let t43283 = 0.17090058289204942853e-2 * t40896;
    let t43286 = 0.53833683610995569986e-1 * t2508 * t2936 * t28024;
    let t43287 = -t43243 - 0.46143157380853345702e-1 * t43244 - 0.46143157380853345702e-1 * t43248 + 0.76905262301422242837e-2 * t2508 * t779 * t13172 + t43254 + t43257 + t43260 + t43263 + t43265 - t43267 - t43269 - 0.23071578690426672851e-1 * t2508 * t1901 * t43270 - t43274 + t43275 + 0.15381052460284448567e-1 * t43278 - t43282 - t43283 - t43286;
    (t43287,)
}

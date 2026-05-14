//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1155/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1155<F: Float>(t100349: F, t5674: F, t93355: F, t1900: F, t457: F, t6: F, t91: F, t11397: F, t93379: F, t38463: F, t5675: F, t10974: F, t100345: F, t22958: F, t100333: F, t100338: F, t100343: F, t100347: F, t92251: F, t92254: F, t92258: F) -> (F, F, F, F, F, F) {
    let t100351 = t5674 * t93355 * t100349;
    let t100356 = t91 * t457 * t6 * t1900;
    let t100358 = t100356 * t93379 * t11397;
    let t100360 = t38463 * t5675;
    let t100362 = t100356 * t100360 * t10974;
    let t100367 = t5674 * t22958 * t100345;
    let t100369 = t100333 / 6.0 + t100338 / 3.0 + t100343 / 3.0 - 2.0 / 9.0 * t100347 + t100351 / 3.0 + 4.0 / 27.0 * t92251 + 4.0 / 9.0 * t100358 - 4.0 / 27.0 * t100362 + t92254 / 24.0 - t92258 / 36.0 - t100367 / 9.0;
    (t100351, t100356, t100358, t100362, t100367, t100369)
}

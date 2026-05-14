//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1248/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1248<F: Float>(t123825: F, t446: F, t9744: F, t122334: F, t2354: F, t3281: F, t122055: F, t9770: F, t121889: F, t121897: F, t122215: F, t122684: F, t41825: F, t122219: F, t41879: F, t18514: F, t96970: F) -> (F, F, F, F, F, F, F, F, F) {
    let t123980 = t446 * t9744 * t123825;
    let t123983 = t3281 * t2354 * t122334;
    let t123986 = t446 * t9770 * t122055;
    let t123989 = t446 * t9770 * t121889;
    let t123992 = t446 * t2354 * t121897;
    let t123995 = t446 * t9744 * t122215;
    let t123998 = t446 * t41825 * t122684;
    let t124001 = t446 * t41879 * t122219;
    let t124003 = t96970 * t18514;
    (t123980, t123983, t123986, t123989, t123992, t123995, t123998, t124001, t124003)
}

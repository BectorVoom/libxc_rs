//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1161/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1161<F: Float>(t1882: F, t28178: F, t10157: F, t108049: F, t108147: F, t1168: F, t13828: F, t13864: F, t14163: F, t14192: F, t1424: F, t1901: F, t24395: F, t2469: F, t24793: F, t265: F, t28284: F, t28294: F, t446: F, t51853: F, t53797: F, t729: F, t762: F, t97897: F, t97899: F, t97917: F, t97919: F, t97933: F, t97952: F, t98123: F) -> (F,) {
    let t110988 = 2.0 / 9.0 * t1882 * t28178;
    let t111006 = 2.0 / 9.0 * t97897 - t446 * t729 * t13828 * t1424 / 3.0 + 2.0 / 9.0 * t97899 - 4.0 / 9.0 * t1901 * t14163 * t108049 - 2.0 * t446 * t10157 * t265 * t108147 - 2.0 / 3.0 * t1901 * t24793 * t13864 - 4.0 / 9.0 * t97917 - 4.0 / 9.0 * t97919 + t110988 - 2.0 / 81.0 * t97933 + 2.0 / 3.0 * t446 * t729 * t2469 * t28284 + t446 * t729 * t762 * t24395 * t1168 / 3.0 - 4.0 / 3.0 * t1901 * t51853 * t28294 + 4.0 / 9.0 * t53797 * t98123 * t14192 + 8.0 / 81.0 * t97952;
    (t111006,)
}

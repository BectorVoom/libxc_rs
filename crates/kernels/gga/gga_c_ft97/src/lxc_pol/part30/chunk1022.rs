//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1022/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1022<F: Float>(t10703: F, t1212: F, t143789: F, t143823: F, t15195: F, t152844: F, t15299: F, t153509: F, t15369: F, t154083: F, t154090: F, t154221: F, t154225: F, t154235: F, t154240: F, t154242: F, t154256: F, t1901: F, t24898: F, t2749: F, t28496: F, t28859: F, t29071: F, t29185: F, t29293: F, t296: F, t34053: F, t34198: F, t36112: F, t36186: F, t446: F, t6273: F, t6287: F, t684: F, t840: F, t871: F, t99238: F) -> (F,) {
    let t154268 = -t446 * t296 * t154083 / 3.0 - 2.0 / 9.0 * t1901 * t15195 * t34198 - 2.0 / 27.0 * t154090 + 2.0 / 3.0 * t446 * t840 * t28859 * t6287 - 4.0 / 9.0 * t143789 - t446 * t296 * t154221 / 3.0 + 2.0 / 9.0 * t154225 - 4.0 * t1901 * t29071 * t6273 * t28496 - 4.0 / 3.0 * t1901 * t15369 * t24898 * t29293 - 2.0 / 9.0 * t154235 + 4.0 / 3.0 * t446 * t296 * t153509 + 2.0 / 9.0 * t154240 - 2.0 / 3.0 * t446 * t296 * t154242 - 2.0 / 9.0 * t143823 + t446 * t840 * t871 * t34053 * t1212 / 3.0 + t446 * t840 * t2749 * t36186 / 3.0 + 2.0 / 27.0 * t154256 - 2.0 / 9.0 * t1901 * t10703 * t36112 * t684 - 2.0 / 9.0 * t1901 * t15299 * t152844 - 2.0 / 9.0 * t1901 * t99238 * t29185;
    (t154268,)
}

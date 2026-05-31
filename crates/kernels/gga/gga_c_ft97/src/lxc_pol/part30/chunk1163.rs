//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1163/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1163<F: Float>(t1882: F, t36208: F, t28859: F, t6386: F, t36183: F, t8392: F, t10703: F, t1212: F, t143789: F, t143823: F, t15195: F, t152844: F, t15299: F, t153509: F, t15369: F, t154083: F, t154090: F, t154221: F, t154225: F, t154235: F, t1901: F, t24898: F, t2749: F, t28496: F, t29071: F, t29185: F, t29293: F, t296: F, t34053: F, t34198: F, t36112: F, t36186: F, t446: F, t6273: F, t6287: F, t684: F, t840: F, t871: F, t99238: F) -> (F, F) {
    let t154240 = t1882 * t36208;
    let t154242 = t28859 * t6386;
    let t154256 = t8392 * t36183;
    let t154268 = -t446 * t296 * t154083 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t15195 * t34198 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t154090 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t840 * t28859 * t6287 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t143789 - t446 * t296 * t154221 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t154225 - F::cast_from(4.0_f64) * t1901 * t29071 * t6273 * t28496 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t15369 * t24898 * t29293 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t154235 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t296 * t153509 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t154240 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t296 * t154242 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t143823 + t446 * t840 * t871 * t34053 * t1212 / F::cast_from(3.0_f64) + t446 * t840 * t2749 * t36186 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t154256 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t10703 * t36112 * t684 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t15299 * t152844 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t99238 * t29185;
    (t154242, t154268)
}

//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 999/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk999<F: Float>(t1882: F, t4188: F, t1255: F, t2409: F, t835: F, t10735: F, t10745: F, t10749: F, t10750: F, t10752: F, t15455: F, t15463: F, t15467: F, t15471: F, t15474: F, t15479: F, t15482: F, t15487: F, t1901: F, t446: F) -> F {
    let t15491 = F::new(2.0) / F::new(27.0) * t1882 * t4188;
    let t15493 = t835 * t1255 * t2409;
    let t15496 = F::new(2.0) / F::new(9.0) * t1901 * t15455 - F::new(8.0) / F::new(27.0) * t10735 - t10745 / F::new(9.0) - t10749 - F::new(4.0) / F::new(3.0) * t1901 * t15463 - t15467 + t10750 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t10752 + t15471 - F::new(2.0) / F::new(3.0) * t446 * t15474 - F::new(2.0) * t446 * t15479 - F::new(2.0) * t446 * t15482 - F::new(2.0) / F::new(3.0) * t446 * t15487 + t15491 + F::new(2.0) / F::new(9.0) * t446 * t15493;
    t15496
}

//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 985/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk985<F: Float>(t1882: F, t4248: F, t4301: F, t15136: F, t296: F, t14616: F, t2749: F, t4176: F, t840: F, t4299: F, t824: F, t871: F) -> (F, F, F, F, F, F) {
    let t15271 = F::new(2.0) / F::new(9.0) * t1882 * t4248;
    let t15273 = F::new(2.0) / F::new(9.0) * t1882 * t4301;
    let t15274 = t296 * t15136;
    let t15277 = t296 * t14616;
    let t15281 = t840 * t2749 * t4176;
    let t15284 = t4299 * t824;
    let t15286 = t840 * t871 * t15284;
    (t15271, t15273, t15274, t15277, t15281, t15286)
}

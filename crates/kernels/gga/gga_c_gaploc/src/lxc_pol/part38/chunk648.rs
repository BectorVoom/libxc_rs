//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 648/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk648<F: Float>(t13333: F, t13334: F, t13336: F, t13338: F, t13340: F, t13342: F, t13345: F, t13348: F, t13349: F, t13352: F, t13484: F, t13720: F, t224: F, t5558: F, t744: F, t1980: F, t6110: F) -> (F, F, F, F) {
    let t13721 = -t13333 + t13334 + t13336 - t13338 + t13340 - t13342 + t13345 - t13348 + t13349 - t13352 + t13484;
    let t13722 = t13720 + t13721;
    let t13723 = t224 * t13722;
    let t14537 = t744 * t5558;
    let t15362 = t1980 * t6110;
    (t13722, t13723, t14537, t15362)
}

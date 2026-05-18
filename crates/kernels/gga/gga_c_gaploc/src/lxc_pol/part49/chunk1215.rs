//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1215/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1215<F: Float>(t13953: F, t42520: F, t44243: F, t44245: F, t47113: F, t47115: F, t47121: F, t47785: F, t47788: F, t47790: F, t47791: F, t48241: F, t856: F) -> F {
    let t51219 = t13953 * t856 - t42520 - t44243 - t44245 + t47113 + t47115 + t47121 - t47785 + t47788 - t47790 - t47791 - t48241;
    t51219
}

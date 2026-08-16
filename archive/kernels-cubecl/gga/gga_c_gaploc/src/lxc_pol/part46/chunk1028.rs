//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1028/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1028<F: Float>(t12845: F, t12847: F, t12849: F, t12850: F, t12851: F, t12853: F, t12855: F, t12858: F, t12861: F, t12864: F, t13002: F, t13248: F, t51210: F, t51211: F, t7: F) -> F {
    let tv4rhosigma311 = t12845 - t12847 + t12849 - t12850 - t12851 + t12853 - t12855 - t12858 + t12861 + t12864 - t13002 + t13248 + t7 * (t51210 + t51211);
    tv4rhosigma311
}

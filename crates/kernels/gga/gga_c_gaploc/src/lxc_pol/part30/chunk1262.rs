//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1262/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1262<F: Float>(t22826: F, t3009: F, t590: F, t7068: F, t23516: F, t32616: F, t28072: F, t28075: F, t28079: F, t28081: F, t28085: F, t28089: F, t32839: F, t32843: F, t32846: F, t32850: F, t32853: F, t32856: F, t32860: F, t32866: F) -> F {
    let t32870 = F::cast_from(0.30674340763136599742e1_f64) * t22826 * t3009 * t7068 * t590;
    let t32872 = F::cast_from(0.51123901271894332902e1_f64) * t23516 * t32616;
    let t32873 = -t32839 - t32843 - t32846 - t28072 - t28075 - t28079 - t28081 + t28085 - t32850 - t28089 - t32853 + t32856 + t32860 - t32866 - t32870 + t32872;
    t32873
}

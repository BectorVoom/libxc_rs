//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2590/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2590<F: Float>(t51738: F, t51741: F, t51744: F, t51884: F, t51889: F, t51892: F, t51898: F, t51903: F, t51905: F, t51913: F, t51916: F, t51641: F, t51669: F, t51736: F, t51859: F, t51862: F, t51864: F, t51866: F, t51870: F, t51874: F, t51880: F, t52450: F, t52451: F, t52453: F) -> F {
    let t52455 = t51738 + t51741 + t51744 + t51884 - t51889 + t51892 - t51898 - t51903 - t51905 + t51913 - t51916;
    let t52458 = t52450 + t52451 + t52453 + t51859 + t51862 - t51864 - t51866 - t51870 - t51874 + t51641 + t51669 + t51880 + t51736 + t52455;
    t52458
}

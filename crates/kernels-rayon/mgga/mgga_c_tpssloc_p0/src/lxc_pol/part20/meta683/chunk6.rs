//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2590/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2590(t51738: f64, t51741: f64, t51744: f64, t51884: f64, t51889: f64, t51892: f64, t51898: f64, t51903: f64, t51905: f64, t51913: f64, t51916: f64, t51641: f64, t51669: f64, t51736: f64, t51859: f64, t51862: f64, t51864: f64, t51866: f64, t51870: f64, t51874: f64, t51880: f64, t52450: f64, t52451: f64, t52453: f64) -> f64 {
    let t52455 = t51738 + t51741 + t51744 + t51884 - t51889 + t51892 - t51898 - t51903 - t51905 + t51913 - t51916;
    let t52458 = t52450 + t52451 + t52453 + t51859 + t51862 - t51864 - t51866 - t51870 - t51874 + t51641 + t51669 + t51880 + t51736 + t52455;
    t52458
}

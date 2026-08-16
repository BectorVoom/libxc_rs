//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2638/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2638(t11944: f64, t1256: f64, t14696: f64, t15838: f64, t1763: f64, t193: f64, t336: f64, t3633: f64, t43706: f64, t4700: f64, t51889: f64, t51892: f64, t51898: f64, t51903: f64, t51905: f64, t51906: f64, t51913: f64, t51916: f64, t51946: f64, t53665: f64, t53697: f64, t53729: f64) -> f64 {
    let t53735 = -t51889 + t51892 - 6.0_f64 * t4700 * t1763 * t43706 * t11944 - t51898 - 3.0_f64 * t4700 * t14696 * t3633 - t51903 - t51905 + 6.0_f64 * t4700 * t15838 * t51906 + t51913 - t51916 + t193 * t336 * (t51946 + t53665 + t53697 + t53729) * t1256;
    t53735
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2694/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2694(t16028: f64, t225: f64, t12022: f64, t12437: f64, t12438: f64, t1375: f64, t1386: f64, t16437: f64, t16460: f64, t16471: f64, t16475: f64, t1842: f64, t1843: f64, t3758: f64, t3887: f64, t3912: f64, t39913: f64, t39916: f64, t39919: f64, t40591: f64, t5215: f64, t53866: f64, t539: f64, t54817: f64, t568: f64) -> f64 {
    let t54825 = t16028 * t225;
    let t54832 = 24.0_f64 * t12022 * t1375 * t1842 * t40591 + 2.0_f64 * t12437 * t1375 * t1842 * t3887 + t539 * t54817 * t568 - t12438 * t5215 - 6.0_f64 * t1386 * t53866 - 3.0_f64 * t1386 * t54825 - 3.0_f64 * t16437 * t3758 - 3.0_f64 * t16460 * t3912 + 6.0_f64 * t16471 * t3758 - 18.0_f64 * t16475 * t3758 - 3.0_f64 * t1843 * t39913 - 3.0_f64 * t1843 * t39916 - t1843 * t39919;
    t54832
}

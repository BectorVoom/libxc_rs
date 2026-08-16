//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1063/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1063(t124673: f64, t125050: f64, t125053: f64, t125065: f64, t125067: f64, t125068: f64, t125069: f64, t125071: f64, t130275: f64, t130495: f64, t130498: f64, t1398: f64, t1852: f64, t1858: f64, t2099: f64, t2105: f64, t2170: f64, t2174: f64, t29396: f64, t29430: f64, t29866: f64, t29884: f64, t3: f64, t34176: f64, t34194: f64, t580: f64, t6471: f64, t6483: f64, t7946: f64, t7961: f64, t8111: f64, t8119: f64, t8844: f64, t8852: f64) -> f64 {
    let tv4rho2sigma216 = t6471 * t8852 + 2.0_f64 * t7946 * t8119 + 2.0_f64 * t124673 + t1398 * (t130275 + t130498) + 2.0_f64 * t125069 + t29866 * t2105 + t3 * t130495 * t580 + t2099 * t29884 + 2.0_f64 * t125065 + 2.0_f64 * t125067 + t2170 * t29430 + 2.0_f64 * t125071 + 2.0_f64 * t8111 * t7961 + t29396 * t2174 + 2.0_f64 * t125053 + 2.0_f64 * t125050 + 2.0_f64 * t125068 + 2.0_f64 * t34176 * t1858 + 2.0_f64 * t1852 * t34194 + t8844 * t6483;
    tv4rho2sigma216
}

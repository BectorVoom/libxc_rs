//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1226/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1226(t119853: f64, t22574: f64, t8643: f64, t31048: f64, t7685: f64, t31033: f64, t119830: f64, t119831: f64, t119835: f64, t119837: f64, t119839: f64, t119841: f64, t119844: f64, t119845: f64, t119850: f64, t119852: f64, t31224: f64, t32674: f64, t32676: f64, t4073: f64, t5107: f64, t6515: f64, t7670: f64, t8313: f64) -> f64 {
    let t119856 = 6.0_f64 * t22574 * t8643 * t119853;
    let t119858 = 3.0_f64 * t7685 * t31048;
    let t119862 = t7685 * t31033;
    let t119863 = -2.0_f64 * t31224 * t4073 - t5107 * t8313 - 2.0_f64 * t6515 * t7670 - t119830 + t119831 + t119835 - t119837 - t119839 - t119841 - t119844 - 2.0_f64 * t119845 - t119850 - t119852 - t119856 + t119858 - t119862 - t32674 - t32676;
    t119863
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 787/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk787<F: Float>(t1591: F, t2093: F, t2109: F, t4314: F, t1615: F, t1592: F, t1617: F, t2110: F, t4117: F, t4399: F, t4409: F, t4414: F, t5750: F, t5754: F, t5759: F, t5762: F, t5764: F, t5766: F, t5771: F, t5774: F, t5777: F, t5783: F, t5873: F, t5878: F, t5883: F, t5892: F) -> (F, F, F, F) {
    let t6193 = t2093 * t1591;
    let t6207 = t2109 * t4314;
    let t6208 = t6207 * t1615;
    let t6219 = -F::cast_from(0.66725e-1_f64) * t6193 * t1617 - F::cast_from(0.66725e-1_f64) * t4409 * t2110 - F::cast_from(0.17411041666666666666e-2_f64) * t5750 + F::cast_from(0.11607361111111111111e-2_f64) * t5754 - F::cast_from(0.30952962962962962962e-2_f64) * t5759 + F::cast_from(0.11607361111111111111e-2_f64) * t5762 + F::cast_from(0.77382407407407407407e-3_f64) * t5764 - F::cast_from(0.11607361111111111111e-2_f64) * t5766 - t4399 + F::cast_from(0.11607361111111111111e-2_f64) * t4117 - F::cast_from(0.30952962962962962962e-2_f64) * t5771 + F::cast_from(0.11607361111111111111e-2_f64) * t5774 + F::cast_from(0.890445125e-2_f64) * t4414 * t6208 + F::cast_from(0.66725e-1_f64) * t1592 * t6208 + F::cast_from(0.11607361111111111111e-2_f64) * t5777 - F::cast_from(0.23214722222222222222e-2_f64) * t5783 + F::cast_from(0.17411041666666666666e-2_f64) * t5873 - F::cast_from(0.38691203703703703703e-3_f64) * t5878 - F::cast_from(0.11607361111111111111e-2_f64) * t5883 - F::cast_from(0.46429444444444444443e-2_f64) * t5892;
    (t6193, t6207, t6208, t6219)
}

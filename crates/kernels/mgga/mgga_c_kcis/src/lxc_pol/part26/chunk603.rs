//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 603/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk603<F: Float>(t1615: F, t6207: F, t1592: F, t1617: F, t2110: F, t4117: F, t4399: F, t4409: F, t4414: F, t5750: F, t5754: F, t5759: F, t5762: F, t5764: F, t5766: F, t5771: F, t5774: F, t5777: F, t5783: F, t5873: F, t5878: F, t5883: F, t5892: F, t6193: F) -> (F, F) {
    let t6208 = t6207 * t1615;
    let t6219 = -F::new(0.66725e-1) * t6193 * t1617 - F::new(0.66725e-1) * t4409 * t2110 - F::new(0.17411041666666666666e-2) * t5750 + F::new(0.11607361111111111111e-2) * t5754 - F::new(0.30952962962962962962e-2) * t5759 + F::new(0.11607361111111111111e-2) * t5762 + F::new(0.77382407407407407407e-3) * t5764 - F::new(0.11607361111111111111e-2) * t5766 - t4399 + F::new(0.11607361111111111111e-2) * t4117 - F::new(0.30952962962962962962e-2) * t5771 + F::new(0.11607361111111111111e-2) * t5774 + F::new(0.890445125e-2) * t4414 * t6208 + F::new(0.66725e-1) * t1592 * t6208 + F::new(0.11607361111111111111e-2) * t5777 - F::new(0.23214722222222222222e-2) * t5783 + F::new(0.17411041666666666666e-2) * t5873 - F::new(0.38691203703703703703e-3) * t5878 - F::new(0.11607361111111111111e-2) * t5883 - F::new(0.46429444444444444443e-2) * t5892;
    (t6208, t6219)
}

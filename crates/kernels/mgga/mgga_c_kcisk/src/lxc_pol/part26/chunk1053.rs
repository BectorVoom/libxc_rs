//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1053/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1053<F: Float>(t14612: F, t8288: F, t1596: F, t13956: F, t14160: F, t19788: F, t19806: F, t21420: F, t21425: F, t21426: F, t21438: F, t21440: F, t21446: F, t21449: F, t26851: F, t26858: F, t26862: F, t26867: F, t26869: F, t26873: F, t26878: F, t4347: F) -> (F, F, F) {
    let t27958 = t8288 * t14612;
    let t27959 = t27958 * t1596;
    let t27962 = -0.77382407407407407407e-3 * t19788 - 0.11607361111111111111e-1 * t26851 - t21420 + 0.46429444444444444444e-2 * t19806 - t21425 + t21426 - 0.25794135802469135802e-3 * t13956 - 0.19345601851851851852e-2 * t26858 - 0.11607361111111111111e-2 * t26862 - t21438 + t21440 - t21446 + t21449 - 0.30952962962962962963e-2 * t26867 - 0.23214722222222222221e-2 * t26869 - 0.51588271604938271603e-3 * t26873 - 0.30952962962962962962e-2 * t26878 - 0.38691203703703703703e-3 * t14160 - 0.223494e0 * t4347 * t27959;
    (t27958, t27959, t27962)
}

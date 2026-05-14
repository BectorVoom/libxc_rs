//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1184/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1184<F: Float>(t95571: F, t27014: F, t28093: F, t95587: F, t1250: F, t251: F, t47652: F, t2888: F, t7773: F, t4566: F, t96737: F, t1662: F, t26997: F, t92693: F, t26955: F, t26960: F, t26977: F, t27020: F, t28204: F, t95569: F, t95579: F, t95581: F, t95585: F, t95626: F) -> (F, F, F) {
    let t96779 = 0.25794135802469135802e-2 * t95571;
    let t96781 = 0.23168402777777777778e-3 * t27014 * t28093;
    let t96787 = 0.15476481481481481481e-2 * t95587;
    let t96790 = t47652 * t251 * t1250;
    let t96793 = t2888 * t7773;
    let t96795 = t96793 * t4566 * t96737;
    let t96799 = t92693 * t1662 * t26997;
    let t96802 = 0.69644166666666666666e-2 * t95569 + t96779 + t96781 + 0.51588271604938271604e-3 * t95579 - 0.41270617283950617284e-2 * t95581 + 0.46377350260416666667e-4 * t28204 * t27020 + 0.46429444444444444443e-2 * t95585 - t96787 - 0.38691203703703703703e-3 * t95626 - 0.92835860883789062501e-5 * t96790 * t26977 + 0.41224311342592592592e-4 * t26955 * t96795 - 0.23168402777777777778e-3 * t26960 * t96799;
    (t96795, t96799, t96802)
}

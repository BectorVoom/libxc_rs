//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1067/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1067<F: Float>(t1347: F, t5586: F, t1563: F, t6072: F, t1911: F, t3918: F, t16050: F, t187: F, t15934: F, t15988: F, t16631: F, t16719: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17889 = t5586 * t1347;
    let t17892 = t6072 * t1563;
    let t17895 = t1911 * t3918;
    let t17905 = F::new(0.2283111111111111111e-1) * t16050;
    let t17942 = t187 * t5586;
    let t17973 = F::new(0.15476481481481481481e-2) * t15934;
    let t17995 = F::new(0.23214722222222222222e-2) * t15988;
    let t18002 = F::new(0.23214722222222222222e-2) * t16631;
    let t18037 = F::new(0.15476481481481481481e-2) * t16719;
    (t17889, t17892, t17895, t17905, t17942, t17973, t17995, t18002, t18037)
}

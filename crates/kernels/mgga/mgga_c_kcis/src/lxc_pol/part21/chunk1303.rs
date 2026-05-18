//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1303/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1303<F: Float>(t26728: F, t27856: F, t13376: F, t4947: F, t922: F, t2829: F, t4781: F, t26748: F, t27773: F, t27958: F, t2845: F, t2894: F, t7703: F, t8034: F, t92872: F, t92908: F, t92929: F, t93526: F, t93709: F, t9933: F) -> (F, F, F) {
    let t95963 = t26728 * t27856;
    let t95976 = t4947 * t13376 * t922;
    let t95980 = t4947 * t4781 * t2829;
    let t95983 = F::new(0.14739506172839506172e-2) * t92872 + F::new(0.15445601851851851852e-3) * t93526 - F::new(0.58958024691358024689e-2) * t92908 - F::new(0.22109259259259259258e-2) * t92929 - F::new(0.4946917361111111111e-3) * t93709 * t8034 + F::new(0.61836467013888888888e-4) * t95963 + F::new(0.23168402777777777778e-3) * t7703 * t2894 * t27773 * t2829 + F::new(0.30891203703703703704e-3) * t7703 * t9933 * t27773 * t2845 + F::new(0.46336805555555555556e-3) * t26748 * t27958 + F::new(0.46336805555555555556e-3) * t7703 * t95976 + F::new(0.23168402777777777778e-3) * t7703 * t95980;
    (t95976, t95980, t95983)
}

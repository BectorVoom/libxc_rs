//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1351/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1351<F: Float>(t26954: F, t28203: F, t7773: F, t993: F, t4580: F, t96737: F, t15573: F, t28152: F, t7788: F, t95868: F, t27055: F, t28190: F) -> (F, F, F, F, F, F) {
    let t96926 = t28203 * t26954;
    let t96935 = t993 * t7773;
    let t96937 = t96935 * t4580 * t96737;
    let t96940 = t15573 * t28152;
    let t96942 = F::new(0.23168402777777777778e-3) * t7788 * t96940;
    let t96943 = F::new(0.15476481481481481481e-2) * t95868;
    let t96945 = F::new(0.23168402777777777778e-3) * t28190 * t27055;
    (t96926, t96937, t96940, t96942, t96943, t96945)
}

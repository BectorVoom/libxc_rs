//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1121/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1121<F: Float>(t1014: F, t28919: F, t28973: F, t19727: F, t3200: F, t95926: F, t19711: F, t4554: F, t1087: F, t303: F, t6556: F, t19656: F, t356: F, t1774: F, t4924: F, t100078: F, t11072: F, t1268: F, t26960: F, t28098: F, t6774: F, t922: F, t96917: F, t97193: F) -> (F, F, F, F, F, F, F, F) {
    let t100578 = t1014 * t28919;
    let t100580 = t1014 * t28973;
    let t100583 = t3200 * t95926 * t19727;
    let t100586 = t4554 * t95926 * t19711;
    let t100596 = t303 * t1087 * t6556;
    let t100599 = t303 * t356 * t19656;
    let t100602 = t303 * t4924 * t1774;
    let t100606 = 0.11607361111111111111e-2 * t100578 - t97193 - 0.11607361111111111111e-2 * t100580 - 0.30952962962962962962e-2 * t100583 + 0.25794135802469135802e-2 * t100586 - 0.23168402777777777778e-3 * t26960 * t11072 * t1268 * t6774 * t922 + 0.23168402777777777778e-3 * t96917 * t28098 + 0.11607361111111111111e-2 * t100596 + 0.11607361111111111111e-2 * t100599 + 0.23214722222222222222e-2 * t100602 + 0.11584201388888888889e-3 * t26960 * t100078;
    (t100578, t100580, t100583, t100586, t100596, t100599, t100602, t100606)
}

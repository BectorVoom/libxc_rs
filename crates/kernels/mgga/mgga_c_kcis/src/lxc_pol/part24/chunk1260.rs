//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1260/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1260<F: Float>(t1774: F, t303: F, t4924: F, t100078: F, t100578: F, t100580: F, t100583: F, t100586: F, t100596: F, t100599: F, t11072: F, t1268: F, t26960: F, t28098: F, t6774: F, t922: F, t96917: F, t97193: F) -> (F, F) {
    let t100602 = t303 * t4924 * t1774;
    let t100606 = F::new(0.11607361111111111111e-2) * t100578 - t97193 - F::new(0.11607361111111111111e-2) * t100580 - F::new(0.30952962962962962962e-2) * t100583 + F::new(0.25794135802469135802e-2) * t100586 - F::new(0.23168402777777777778e-3) * t26960 * t11072 * t1268 * t6774 * t922 + F::new(0.23168402777777777778e-3) * t96917 * t28098 + F::new(0.11607361111111111111e-2) * t100596 + F::new(0.11607361111111111111e-2) * t100599 + F::new(0.23214722222222222222e-2) * t100602 + F::new(0.11584201388888888889e-3) * t26960 * t100078;
    (t100602, t100606)
}

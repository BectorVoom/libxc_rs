//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 859/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk859<F: Float>(t8815: F, t9435: F, t9438: F, t8822: F, t9488: F, t8832: F, t8837: F, t8844: F, t8846: F, t8852: F, t8856: F, t8860: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42518 = F::new(0.5107751987195740728e-4) * t8815;
    let t42519 = F::new(0.4726e1) * t9435;
    let t42520 = F::new(0.11974241701863808564e0) * t9438;
    let t42521 = F::new(0.5987120850931904282e-1) * t8822;
    let t42527 = F::new(0.39914139006212695214e-1) * t9488;
    let t42528 = F::new(0.638468998399467591e-4) * t8832;
    let t42529 = F::new(0.638468998399467591e-4) * t8837;
    let t42530 = F::new(0.212822999466489197e-4) * t8844;
    let t42531 = F::new(0.212822999466489197e-4) * t8846;
    let t42534 = F::new(0.60975299583150056624e-3) * t8852;
    let t42535 = F::new(0.60975299583150056624e-3) * t8856;
    let t42536 = F::new(0.60975299583150056624e-3) * t8860;
    (t42518, t42519, t42520, t42521, t42527, t42528, t42529, t42530, t42531, t42534, t42535, t42536)
}

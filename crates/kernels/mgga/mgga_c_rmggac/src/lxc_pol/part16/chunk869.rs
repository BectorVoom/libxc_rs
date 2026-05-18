//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 869/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk869<F: Float>(t41146: F, t41160: F, t41170: F, t41195: F, t41297: F, t41308: F, t41314: F, t41319: F, t41323: F, t41338: F, t41347: F, t41371: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t43507 = F::new(0.3193131120497015617e0) * t41146;
    let t43513 = F::new(0.14161231045397953428e-1) * t41160;
    let t43518 = F::new(0.21241846568096930142e-1) * t41170;
    let t43530 = F::new(0.15965655602485078085e0) * t41195;
    let t43588 = F::new(0.24244143692662525982e0) * t41297;
    let t43592 = F::new(0.14546486215597515589e0) * t41308;
    let t43594 = F::new(0.14546486215597515589e0) * t41314;
    let t43596 = F::new(0.4838420607177634088e-2) * t41319;
    let t43598 = F::new(0.67737888500486877232e-2) * t41323;
    let t43606 = F::new(0.31931311204970156172e0) * t41338;
    let t43611 = F::new(0.9676841214355268176e-3) * t41347;
    let t43628 = F::new(0.10643770401656718724e0) * t41371;
    (t43507, t43513, t43518, t43530, t43588, t43592, t43594, t43596, t43598, t43606, t43611, t43628)
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 897/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk897<F: Float>(t8815: F, t9435: F, t9438: F, t8822: F, t42515: F, t42516: F, t42517: F, t8183: F, t8184: F, t8187: F, t8190: F, t9488: F, t8832: F, t8837: F, t8844: F, t8846: F) -> (F, F, F, F, F, F) {
    let t42518 = 0.5107751987195740728e-4 * t8815;
    let t42519 = 0.4726e1 * t9435;
    let t42520 = 0.11974241701863808564e0 * t9438;
    let t42521 = 0.5987120850931904282e-1 * t8822;
    let t42522 = -t8183 + t8184 + t8187 + t42515 - t8190 + t42516 + t42517 - t42518 - t42519 - t42520 + t42521;
    let t42527 = 0.39914139006212695214e-1 * t9488;
    let t42528 = 0.638468998399467591e-4 * t8832;
    let t42529 = 0.638468998399467591e-4 * t8837;
    let t42530 = 0.212822999466489197e-4 * t8844;
    let t42531 = 0.212822999466489197e-4 * t8846;
    (t42522, t42527, t42528, t42529, t42530, t42531)
}

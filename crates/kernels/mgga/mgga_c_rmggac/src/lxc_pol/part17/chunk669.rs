//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 669/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk669<F: Float>(t8716: F, t8718: F, t8735: F, t8737: F, t8741: F, t8832: F, t8837: F, t8844: F, t8846: F, t8872: F, t9001: F, t9009: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9447 = F::new(0.18183107769496894486e-1) * t8716;
    let t9448 = F::new(0.24244143692662525982e-1) * t8718;
    let t9457 = F::new(0.17701538806747441785e-2) * t8735;
    let t9458 = F::new(0.21241846568096930142e-2) * t8737;
    let t9460 = F::new(0.53218852008283593619e-1) * t8741;
    let t9490 = F::new(0.3192344991997337955e-4) * t8832;
    let t9491 = F::new(0.3192344991997337955e-4) * t8837;
    let t9492 = F::new(0.1064114997332445985e-4) * t8844;
    let t9493 = F::new(0.1064114997332445985e-4) * t8846;
    let t9501 = F::new(0.8980681276397856423e-1) * t8872;
    let t9583 = F::new(0.15965655602485078085e0) * t9001;
    let t9586 = F::new(0.23948483403727617128e0) * t9009;
    (t9447, t9448, t9457, t9458, t9460, t9490, t9491, t9492, t9493, t9501, t9583, t9586)
}

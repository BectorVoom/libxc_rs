//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 693/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk693<F: Float>(t7595: F, t7628: F, t7663: F, t8739: F, t9458: F, t9460: F, t9904: F, t9906: F, t9909: F, t9911: F, t9913: F, t9915: F, t9917: F, t9919: F, t9921: F, t9923: F) -> F {
    let t9925 = -t9458 - F::cast_from(0.79828278012425390428e-1_f64) * t8739 + t9460 + t7595 - F::cast_from(0.2727466165424534173e-1_f64) * t9904 - F::cast_from(0.12700854093841289481e-2_f64) * t9906 - t7628 - F::cast_from(0.99785347515531738034e-2_f64) * t9909 + F::cast_from(0.39828462315181744016e-3_f64) * t9911 - F::cast_from(0.33190385262651453347e-3_f64) * t9913 - F::cast_from(0.99785347515531738034e-2_f64) * t9915 + F::cast_from(0.14967802127329760705e-1_f64) * t9917 - F::cast_from(0.27879923620627220811e-2_f64) * t9919 - F::cast_from(0.5987120850931904282e-1_f64) * t9921 - F::cast_from(0.13276154105060581339e-2_f64) * t9923 - t7663;
    t9925
}

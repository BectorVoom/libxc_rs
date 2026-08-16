//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 622/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk622<F: Float>(t7583: F, t8702: F, t8706: F, t8710: F, t8714: F, t8716: F, t8718: F, t8720: F, t8722: F, t8724: F, t8726: F, t797: F, t8700: F) -> (F, F) {
    let t8728 = -F::cast_from(0.45457769423742236216e-2_f64) * t8702 + F::cast_from(0.9072038638458063915e-4_f64) * t8706 - F::cast_from(0.2419210303588817044e-3_f64) * t8710 + F::cast_from(0.28224120208536198848e-3_f64) * t8714 - F::cast_from(0.90915538847484472432e-2_f64) * t8716 + F::cast_from(0.12122071846331262991e-1_f64) * t8718 - F::cast_from(0.10584045078201074568e-3_f64) * t8720 + F::cast_from(0.34093327067806677162e-2_f64) * t8722 + F::cast_from(0.19914231157590872008e-2_f64) * t8724 - F::cast_from(0.27879923620627220811e-2_f64) * t8726 + t7583;
    let t8729 = t797 * t8700;
    (t8728, t8729)
}

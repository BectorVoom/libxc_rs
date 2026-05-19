//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 216/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk216<F: Float>(t305: F, t326: F, t344: F, t349: F, t793: F, t794: F, t797: F, t798: F, t833: F, t838: F, t839: F, t848: F, t851: F, t854: F, t861: F) -> F {
    let t866 = F::cast_from(0.39914139006212695214e-1_f64) * t793 * t794 - F::cast_from(0.11974241701863808564e0_f64) * t797 * t798 + F::cast_from(0.19957069503106347607e-1_f64) * t305 * t833 + F::cast_from(0.79828278012425390428e-1_f64) * t838 * t839 - F::cast_from(0.19957069503106347607e-1_f64) * t326 * t848 + F::cast_from(0.13276154105060581339e-2_f64) * t851 * t794 - F::cast_from(0.31862769852145395214e-2_f64) * t854 * t798 + F::cast_from(0.26552308210121162678e-3_f64) * t344 * t833 + F::cast_from(0.18586615747084813875e-2_f64) * t861 * t839 - F::cast_from(0.26552308210121162678e-3_f64) * t349 * t848;
    t866
}

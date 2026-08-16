//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 297/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk297<F: Float>(t1704: F, t1707: F, t1734: F, t1737: F, t1743: F, t305: F, t326: F, t344: F, t349: F, t793: F, t797: F, t838: F, t851: F, t854: F, t861: F) -> F {
    let t1756 = F::cast_from(0.39914139006212695214e-1_f64) * t793 * t1704 - F::cast_from(0.11974241701863808564e0_f64) * t797 * t1707 + F::cast_from(0.19957069503106347607e-1_f64) * t305 * t1734 + F::cast_from(0.79828278012425390428e-1_f64) * t838 * t1737 - F::cast_from(0.19957069503106347607e-1_f64) * t326 * t1743 + F::cast_from(0.13276154105060581339e-2_f64) * t851 * t1704 - F::cast_from(0.31862769852145395214e-2_f64) * t854 * t1707 + F::cast_from(0.26552308210121162678e-3_f64) * t344 * t1734 + F::cast_from(0.18586615747084813875e-2_f64) * t861 * t1737 - F::cast_from(0.26552308210121162678e-3_f64) * t349 * t1743;
    t1756
}

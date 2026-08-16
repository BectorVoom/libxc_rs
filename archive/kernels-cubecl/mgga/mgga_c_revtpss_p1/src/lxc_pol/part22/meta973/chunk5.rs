//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3265/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3265<F: Float>(t10760: F, t18409: F, t9794: F, t10777: F, t10779: F, t5984: F, t837: F, t14749: F, t40673: F, t40737: F, t4450: F, t50771: F, t50773: F, t50784: F, t50957: F, t61959: F, t61969: F, t61973: F, t61977: F) -> F {
    let t61981 = t10760 * t9794 * t18409;
    let t61985 = t10777 * t10779 * t5984 * t837;
    let t61987 = F::cast_from(0.10164000561857065645e-3_f64) * t61959 - F::cast_from(0.10289764348336736873e0_f64) * t50957 * t40673 * t4450 * t14749 - F::cast_from(0.11433071498151929859e-3_f64) * t50771 + F::cast_from(0.54208002996571016772e-3_f64) * t50773 + t40737 + F::cast_from(0.15246000842785598468e-3_f64) * t61969 - F::cast_from(0.57165357490759649296e-4_f64) * t61973 + F::cast_from(0.28582678745379824648e-3_f64) * t61977 - F::cast_from(0.25410001404642664112e-4_f64) * t50784 - F::cast_from(0.45178982497454656791e-5_f64) * t61981 + F::cast_from(0.10164000561857065645e-3_f64) * t61985;
    t61987
}

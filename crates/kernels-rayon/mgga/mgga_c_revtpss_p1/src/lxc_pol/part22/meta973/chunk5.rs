//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3265/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3265(t10760: f64, t18409: f64, t9794: f64, t10777: f64, t10779: f64, t5984: f64, t837: f64, t14749: f64, t40673: f64, t40737: f64, t4450: f64, t50771: f64, t50773: f64, t50784: f64, t50957: f64, t61959: f64, t61969: f64, t61973: f64, t61977: f64) -> f64 {
    let t61981 = t10760 * t9794 * t18409;
    let t61985 = t10777 * t10779 * t5984 * t837;
    let t61987 = 0.10164000561857065645e-3_f64 * t61959 - 0.10289764348336736873e0_f64 * t50957 * t40673 * t4450 * t14749 - 0.11433071498151929859e-3_f64 * t50771 + 0.54208002996571016772e-3_f64 * t50773 + t40737 + 0.15246000842785598468e-3_f64 * t61969 - 0.57165357490759649296e-4_f64 * t61973 + 0.28582678745379824648e-3_f64 * t61977 - 0.25410001404642664112e-4_f64 * t50784 - 0.45178982497454656791e-5_f64 * t61981 + 0.10164000561857065645e-3_f64 * t61985;
    t61987
}

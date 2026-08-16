//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1089/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1089(t37386: f64, t37397: f64, t37406: f64, t37412: f64, t37414: f64, t37442: f64, t37447: f64, t37451: f64, t37458: f64, t37460: f64, t37463: f64, t37472: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39054 = 0.205201155180140685e-5_f64 * t37386;
    let t39059 = 0.487802396665200453e-2_f64 * t37397;
    let t39061 = 0.11709622077411463733e-2_f64 * t37406;
    let t39062 = 0.18292589874945016987e-2_f64 * t37412;
    let t39064 = 0.18292589874945016987e-2_f64 * t37414;
    let t39069 = 0.91462949374725084936e-3_f64 * t37442;
    let t39071 = 0.18292589874945016987e-2_f64 * t37447;
    let t39072 = 0.13911401682674235141e-1_f64 * t37451;
    let t39074 = 0.91462949374725084936e-3_f64 * t37458;
    let t39075 = 0.91462949374725084936e-3_f64 * t37460;
    let t39076 = 0.13010691197123848592e-3_f64 * t37463;
    let t39081 = 0.89430439388620083049e-2_f64 * t37472;
    (t39054, t39059, t39061, t39062, t39064, t39069, t39071, t39072, t39074, t39075, t39076, t39081)
}

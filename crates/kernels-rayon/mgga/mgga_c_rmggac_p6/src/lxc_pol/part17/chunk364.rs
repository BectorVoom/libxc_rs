//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 364/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk364(t2075: f64, t2118: f64, t2106: f64, t661: f64, t2094: f64, t2096: f64, t2099: f64, t2101: f64, t2104: f64, t2108: f64, t2109: f64, t2111: f64, t2114: f64, t2116: f64) -> (f64, f64) {
    let t2119 = t2118 * t2075;
    let t2121 = t661 * t2106;
    let t2122 = 0.40320171726480284067e-4_f64 * t2121;
    let t2123 = -0.99785347515531738034e-2_f64 * t2094 + 0.14967802127329760705e-1_f64 * t2096 + t2099 + 0.34093327067806677162e-2_f64 * t2101 - 0.45457769423742236216e-2_f64 * t2104 - t2108 - 0.33190385262651453347e-3_f64 * t2109 + 0.39828462315181744016e-3_f64 * t2111 + t2114 + 0.9072038638458063915e-4_f64 * t2116 - 0.10584045078201074568e-3_f64 * t2119 - t2122;
    (t2122, t2123)
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 347/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk347<F: Float>(t2075: F, t2118: F, t2106: F, t661: F, t2094: F, t2096: F, t2099: F, t2101: F, t2104: F, t2108: F, t2109: F, t2111: F, t2114: F, t2116: F) -> (F, F) {
    let t2119 = t2118 * t2075;
    let t2121 = t661 * t2106;
    let t2122 = 0.40320171726480284067e-4 * t2121;
    let t2123 = -0.99785347515531738034e-2 * t2094 + 0.14967802127329760705e-1 * t2096 + t2099 + 0.34093327067806677162e-2 * t2101 - 0.45457769423742236216e-2 * t2104 - t2108 - 0.33190385262651453347e-3 * t2109 + 0.39828462315181744016e-3 * t2111 + t2114 + 0.9072038638458063915e-4 * t2116 - 0.10584045078201074568e-3 * t2119 - t2122;
    (t2122, t2123)
}

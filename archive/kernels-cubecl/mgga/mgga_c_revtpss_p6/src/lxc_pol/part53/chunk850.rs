//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 850/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk850<F: Float>(t11064: F, t1962: F, t1032: F, t1071: F, t7150: F, t11120: F, t359: F, t1982: F, t994: F, t1972: F, t3223: F, t1024: F, t7125: F) -> (F, F, F, F, F, F, F) {
    let t25445 = t1962 * t11064;
    let t25460 = t1071 * t1032;
    let t25461 = t7150 * t25460;
    let t25464 = t11120 * t359;
    let t25473 = t1982 * t25460;
    let t25476 = t994 * t25460;
    let t25490 = t3223 * t1972;
    let t25495 = t1024 * t7125;
    (t25445, t25461, t25464, t25473, t25476, t25490, t25495)
}

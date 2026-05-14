//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1037/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1037<F: Float>(t25460: F, t7150: F, t11120: F, t359: F, t1976: F, t3270: F, t1096: F, t7135: F, t7160: F, t1982: F, t994: F, t3325: F, t3075: F, t7145: F, t3259: F, t1972: F, t3223: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t25461 = t7150 * t25460;
    let t25464 = t11120 * t359;
    let t25465 = t1976 * t3270;
    let t25466 = t25464 * t25465;
    let t25470 = t7160 * t7135 * t1096;
    let t25473 = t1982 * t25460;
    let t25476 = t994 * t25460;
    let t25479 = t1976 * t3325;
    let t25480 = t7160 * t25479;
    let t25483 = t1976 * t3075;
    let t25484 = t7145 * t25483;
    let t25487 = t1982 * t3259;
    let t25490 = t3223 * t1972;
    (t25461, t25464, t25465, t25466, t25470, t25473, t25476, t25479, t25480, t25483, t25484, t25487, t25490)
}

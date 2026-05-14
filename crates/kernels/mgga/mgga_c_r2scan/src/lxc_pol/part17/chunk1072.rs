//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1072/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1072<F: Float>(t37835: F, t37838: F, t38528: F, t38532: F, t39679: F, t39721: F, t39723: F, t39738: F, t43215: F, t43217: F, t43219: F, t43225: F, t39752: F, t39753: F, t39770: F, t39772: F, t41542: F, t41552: F, t43230: F, t43232: F, t43234: F, t43238: F, t43240: F, t43242: F) -> (F, F) {
    let t44288 = -t39679 + 0.87327386630866483588e-2 * t43215 - 0.97574405393827830187e-2 * t43217 - 0.11565819519348392138e-2 * t39721 + 0.32524801797942610062e-3 * t39723 - 0.26198215989259945076e-1 * t43219 + t38528 + t38532 + 0.58544643236296698113e-1 * t37835 + 0.45022119329691164871e0 * t37838 + t39738 + 0.69345773920434148507e0 * t43225;
    let t44297 = -t39752 - t39753 - t41542 + 0.39029762157531132073e-1 * t43230 + 0.87327386630866483588e-2 * t43232 + 0.51220160311720645767e0 * t39770 + 0.25610080155860322883e0 * t43234 - 0.17073386770573548589e1 * t39772 + 0.23115257973478049502e0 * t43238 + t41552 + 0.43663693315433241794e-2 * t43240 - 0.87327386630866483588e-2 * t43242;
    (t44288, t44297)
}

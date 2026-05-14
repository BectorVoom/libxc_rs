//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1223/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1223<F: Float>(t6394: F, t9483: F, t20919: F, t2732: F, t20922: F, t9486: F, t6241: F, t9509: F, t14287: F, t9831: F, t1520: F, t14294: F, t2282: F, t4170: F, t4165: F, t9848: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t33623 = t9483 * t6394;
    let t33624 = t20919 * t2732;
    let t33626 = 2.0 * t20922 * t9486;
    let t33627 = t6241 * t9509;
    let t33629 = 2.0 * t14287 * t9831;
    let t33630 = t9831 * t1520;
    let t33632 = 6.0 * t14294 * t33630;
    let t33633 = t9509 * t2282;
    let t33635 = 2.0 * t4170 * t33633;
    let t33636 = t2732 * t6394;
    let t33638 = 2.0 * t4170 * t33636;
    let t33639 = t4165 * t9848;
    (t33623, t33624, t33626, t33627, t33629, t33630, t33632, t33633, t33635, t33636, t33638, t33639)
}

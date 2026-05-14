//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1000/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1000<F: Float>(t2411: F, t28455: F, t198: F, t206: F, t8019: F, t2718: F, t7398: F, t41040: F, t685: F, t10867: F, t2061: F, t11064: F, t116: F, t29421: F, t1468: F, t1711: F) -> (F, F, F, F, F, F, F, F, F) {
    let t102854 = t28455 * t2411;
    let t102888 = t198 * t206 * t8019;
    let t103059 = t2718 * t7398;
    let t103181 = t685 * t41040;
    let t103452 = t10867 * t2061;
    let t103586 = t8019 * t11064;
    let t104115 = t29421 * t116;
    let t106589 = t11064 * t1468;
    let t107923 = t11064 * t1711;
    (t102854, t102888, t103059, t103181, t103452, t103586, t104115, t106589, t107923)
}

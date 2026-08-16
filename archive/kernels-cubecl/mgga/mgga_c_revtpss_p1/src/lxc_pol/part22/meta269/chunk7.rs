//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1652/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1652<F: Float>(t1343: F, t1450: F, t198: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t3828: F, t4147: F, t532: F, t6777: F, t6778: F, t6779: F, t6780: F, t6781: F, t6802: F, t6816: F, t6836: F, t6922: F) -> F {
    let t6929 = t1450 * t198 * t532 * t6922 - t198 * t4147 * t532 * t6781 + F::cast_from(3.0_f64) * t1343 * t198 * t6816 + F::cast_from(6.0_f64) * t198 * t3828 * t6836 - t2522 - t2562 - t2569 + t2579 + t2587 - t6777 - t6778 + t6779 - t6780 + t6802;
    t6929
}

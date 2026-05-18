//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1282/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1282<F: Float>(t11425: F, t2981: F, t568: F, t2974: F, t34925: F, t8675: F, t1625: F, t5296: F, t1622: F, t1633: F, t11420: F, t116: F, t34021: F) -> (F, F, F, F, F) {
    let t35246 = t11425 * t2981 * t568;
    let t35249 = t34925 * t8675 * t2974;
    let t35251 = t5296 * t1625;
    let t35252 = t1622 * t35251;
    let t35254 = t1633 * t35251;
    let t35257 = t116 * t34021 * t11420;
    (t35246, t35249, t35252, t35254, t35257)
}

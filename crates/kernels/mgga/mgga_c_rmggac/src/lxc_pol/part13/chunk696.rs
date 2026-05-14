//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 696/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk696<F: Float>(t1265: F, t1986: F, t2001: F, t2002: F, t326: F, t333: F, t265: F, t4789: F, t638: F, t71: F, t7311: F, t2007: F, t7939: F, t1982: F, t7428: F, t7547: F) -> (F, F, F, F, F) {
    let t35554 = t1986 * t1265;
    let t35559 = t2001 * t326 * t2002 * t333;
    let t35565 = t638 * t265 * t4789 * t71 * t7311;
    let t35567 = t7939 * t2007;
    let t35577 = t7547 * t7428 * t1982;
    (t35554, t35559, t35565, t35567, t35577)
}

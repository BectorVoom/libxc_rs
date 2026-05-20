//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1522/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1522<F: Float>(t3075: F, t3151: F, t11875: F, t11876: F, t11922: F, t11991: F, t3111: F, t1062: F, t11903: F, t11988: F, t3188: F, t11173: F, t999: F) -> (F, F, F, F, F, F) {
    let t42894 = t3075 * t3151;
    let t42900 = t11875 * t11922 * t11876;
    let t42902 = t11991 * t3111;
    let t42904 = t11903 * t1062;
    let t42907 = t3188 * t11988;
    let t42909 = t11173 * t999;
    (t42894, t42900, t42902, t42904, t42907, t42909)
}

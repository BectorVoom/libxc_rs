//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 679/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk679<F: Float>(t14048: F, t68581: F, t13863: F, t14368: F, t13822: F, t7348: F, t13824: F, t14024: F, t4517: F, t830: F, t14130: F, t1985: F, t3839: F) -> (F, F, F, F, F, F, F) {
    let t68582 = t68581 * t14048;
    let t68602 = t14368 * t13863;
    let t68613 = t13822 * t7348;
    let t68614 = t68613 * t13824;
    let t68621 = t4517 * t830 * t14024;
    let t68622 = t14130 * t68621;
    let t68626 = t1985 * t3839;
    (t68582, t68602, t68613, t68614, t68621, t68622, t68626)
}

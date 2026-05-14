//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 617/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk617<F: Float>(t3813: F, t8715: F, t701: F, t2436: F, t3799: F, t1103: F, t228: F, t231: F, t625: F, t1123: F, t626: F, t1095: F, t694: F, t1152: F, t1771: F, t2345: F, t26: F) -> (F, F, F, F, F, F, F, F) {
    let t13628 = t8715 * t3813;
    let t13629 = t701 * t13628;
    let t13635 = t3799 * t2436;
    let t13636 = 0.1134997482304526749e-1 * t13635;
    let t13643 = t228 * t1103 * t625 * t231;
    let t13647 = t626 * t1123;
    let t13648 = t701 * t13647;
    let t13654 = t694 * t1095;
    let t13680 = t1771 * t1152;
    let t13682 = t26 * t2345;
    (t13629, t13635, t13636, t13643, t13648, t13654, t13680, t13682)
}

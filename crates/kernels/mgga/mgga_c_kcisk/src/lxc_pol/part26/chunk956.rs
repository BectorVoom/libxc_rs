//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 956/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk956<F: Float>(t1312: F, t25993: F, t13830: F, t8048: F, t1322: F, t6204: F, t6205: F, t6211: F, t1308: F, t8021: F, t25446: F, t6175: F, t20111: F, t25465: F, t20116: F, t25469: F) -> (F, F, F, F, F, F, F) {
    let t25994 = t1312 * t25993;
    let t25997 = t13830 * t8048;
    let t25998 = t25997 * t1322;
    let t25999 = t6204 * t25998;
    let t26002 = t6205 * t6211;
    let t26003 = t6204 * t26002;
    let t26008 = t8021 * t1308;
    let t26017 = t6175 * t25446;
    let t26020 = t20111 * t25465;
    let t26023 = t20116 * t25469;
    (t25994, t25999, t26003, t26008, t26017, t26020, t26023)
}

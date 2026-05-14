//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1318/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1318<F: Float>(t1903: F, t5774: F, t4076: F, t6918: F, t72: F, t686: F, t3915: F, t6889: F, t786: F, t1364: F, t14100: F, t5722: F, t1357: F, t6919: F, t689: F, t1444: F) -> (F, F, F, F, F, F) {
    let t22394 = t1903 * t5774;
    let t22395 = t4076 * t22394;
    let t22398 = t6918 * t72;
    let t22399 = t22398 * t686;
    let t22400 = t3915 * t22399;
    let t22404 = t786 * t6889;
    let t22405 = t22404 * t1364;
    let t22407 = t14100 * t5722;
    let t22409 = t1357 * t6919;
    let t22410 = t689 * t22409;
    let t22414 = t6918 * t1444;
    (t22395, t22400, t22405, t22407, t22410, t22414)
}

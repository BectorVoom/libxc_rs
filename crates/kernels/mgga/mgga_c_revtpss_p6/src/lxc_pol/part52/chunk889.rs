//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 889/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk889<F: Float>(t11064: F, t2070: F, t116: F, t7373: F, t13426: F, t1937: F, t18227: F, t4248: F, t6993: F, t7003: F, t1518: F, t648: F) -> (F, F, F, F, F, F, F) {
    let t26590 = t2070 * t11064;
    let t26733 = t116 * t7373;
    let t27116 = F::new(2.0) * t13426 * t1937;
    let t27118 = F::new(2.0) * t18227 * t1937;
    let t27120 = F::new(2.0) * t4248 * t6993;
    let t27122 = F::new(2.0) * t4248 * t7003;
    let t27123 = t648 * t1518;
    (t26590, t26733, t27116, t27118, t27120, t27122, t27123)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2499/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2499<F: Float>(t49957: F, t14322: F, t2496: F, t2609: F, t4186: F, t706: F, t14616: F, t2619: F, t198: F, t775: F, t10565: F, t1469: F) -> (F, F, F, F, F, F) {
    let t49958 = F::cast_from(0.17544670867903938621e1_f64) * t49957;
    let t49963 = t14322 * t2496;
    let t49964 = F::cast_from(0.51947577317044391276e2_f64) * t49963;
    let t49981 = t706 * t2609 * t4186;
    let t49982 = F::new(12.0) * t49981;
    let t50047 = t14616 * t2619;
    let t50048 = F::cast_from(0.73245789224026180216e-3_f64) * t50047;
    let t50080 = t198 * t775;
    let t50084 = t706 * t10565 * t1469;
    (t49958, t49964, t49982, t50048, t50080, t50084)
}

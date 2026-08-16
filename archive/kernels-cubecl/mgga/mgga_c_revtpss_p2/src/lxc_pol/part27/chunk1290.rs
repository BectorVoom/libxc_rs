//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1290/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1290<F: Float>(t13435: F, t7003: F, t2322: F, t25856: F, t25188: F, t7313: F, t508: F, t651: F, t94991: F, t2014: F, t25177: F, t7312: F) -> (F, F, F, F, F) {
    let t95040 = F::cast_from(12.0_f64) * t13435 * t7003;
    let t95042 = F::cast_from(6.0_f64) * t2322 * t25856;
    let t95046 = F::cast_from(3.0_f64) * t25188 * t7313;
    let t95049 = F::cast_from(2.0_f64) * t651 * t508 * t94991;
    let t95056 = F::cast_from(6.0_f64) * t2014 * t7312 * t25177;
    (t95040, t95042, t95046, t95049, t95056)
}

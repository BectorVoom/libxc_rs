//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 930/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk930<F: Float>(t265: F, t393: F, t1100: F, t1102: F, t1699: F, t198: F, t25709: F, t25713: F, t27708: F, t27712: F, t27717: F, t27754: F, t336: F, t5019: F, t5023: F, t7181: F) -> F {
    let t394 = t265 < t393;
    let t27755 = piecewise3::<f64>(t394, t1102 * t198 * t27708 * t336 - t1100 * t27712 * t5023 - t1699 * t25709 * t5023 + F::new(2.0) * t25713 * t27717 * t5023 - t5019 * t5023 * t7181, t27754);
    t27755
}

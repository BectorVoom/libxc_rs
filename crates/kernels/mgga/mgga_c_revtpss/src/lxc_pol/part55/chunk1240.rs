//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1240/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1240<F: Float>(t128302: F, t2055: F, t28042: F, t93: F, t34321: F, t7373: F, t32392: F, t7983: F, t32655: F, t28683: F, t8692: F, t32385: F, t7732: F) -> (F, F, F, F, F, F, F) {
    let t128358 = t128302 * t2055;
    let t128359 = t93 * t28042;
    let t128360 = t128359 * t2055;
    let t128361 = t34321 * t7373;
    let t128362 = t32392 * t7983;
    let t128363 = t32655 * t7983;
    let t128367 = F::new(2.0) * t8692 * t28683;
    let t128483 = F::new(2.0) * t7732 * t32385;
    (t128358, t128360, t128361, t128362, t128363, t128367, t128483)
}

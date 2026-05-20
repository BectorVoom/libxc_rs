//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2611/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2611<F: Float>(t5977: F, t836: F, t10811: F, t18462: F, t18466: F, t125: F, t18615: F, t10744: F, t18418: F, t808: F, t18446: F, t10886: F, t18599: F) -> (F, F, F, F, F, F, F) {
    let t61756 = t5977 * t836;
    let t61774 = t10811 * t18462;
    let t61776 = t10811 * t18466;
    let t61791 = t125 * t18615;
    let t61797 = t10744 * t808 * t18418;
    let t61817 = t10811 * t18446;
    let t61833 = t10886 * t808 * t18599;
    (t61756, t61774, t61776, t61791, t61797, t61817, t61833)
}

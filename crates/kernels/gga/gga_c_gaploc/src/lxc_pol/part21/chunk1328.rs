//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1328/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1328<F: Float>(t10513: F, t20800: F, t4391: F, t10525: F, t1564: F, t539: F, t32033: F, t6716: F, t6717: F, t10533: F, t20796: F, t1397: F, t8237: F, t9287: F) -> (F, F, F, F, F) {
    let t34668 = F::new(0.57200783922960682671e1) * t4391 * t20800 * t10513;
    let t34672 = F::new(0.28600391961480341335e1) * t10525 * t539 * t1564 * t10513;
    let t34675 = F::new(0.12423108009070322895e3) * t6716 * t6717 * t32033;
    let t34678 = F::new(0.55213813373645879534e2) * t20796 * t10533 * t32033;
    let t34680 = t1397 * t8237 * t9287;
    (t34668, t34672, t34675, t34678, t34680)
}

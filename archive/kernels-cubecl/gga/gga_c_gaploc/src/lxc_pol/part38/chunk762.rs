//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 762/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk762<F: Float>(t1853: F, t3615: F, t1022: F, t2925: F, t7290: F, t35450: F, t11576: F, t296: F, t2101: F, t3614: F, t835: F, t1023: F, t35385: F) -> (F, F, F, F, F, F, F) {
    let t35583 = t3615 * t1853;
    let t35610 = t1022 * t2925;
    let t35611 = t7290 * t35610;
    let t35623 = t7290 * t35450;
    let t35659 = t296 * t11576;
    let t35682 = t2101 * t3614;
    let t35709 = t835 * t11576;
    let t35719 = t1023 * t35385;
    (t35583, t35611, t35623, t35659, t35682, t35709, t35719)
}

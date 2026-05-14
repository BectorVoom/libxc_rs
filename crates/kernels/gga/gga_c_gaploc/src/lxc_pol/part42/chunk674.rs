//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 674/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk674<F: Float>(t35610: F, t7290: F, t35450: F, t11576: F, t296: F, t2101: F, t3614: F, t835: F, t1023: F, t35385: F, t1381: F, t3549: F, t11699: F, t747: F, t3516: F, t475: F) -> (F, F, F, F, F, F, F, F, F) {
    let t35611 = t7290 * t35610;
    let t35623 = t7290 * t35450;
    let t35659 = t296 * t11576;
    let t35682 = t2101 * t3614;
    let t35709 = t835 * t11576;
    let t35719 = t1023 * t35385;
    let t35770 = t3549 * t1381;
    let t35781 = t11699 * t747;
    let t35845 = t3516 * t475;
    (t35611, t35623, t35659, t35682, t35709, t35719, t35770, t35781, t35845)
}

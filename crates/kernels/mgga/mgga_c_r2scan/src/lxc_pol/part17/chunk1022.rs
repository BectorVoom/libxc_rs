//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1022/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1022<F: Float>(t12476: F, t37685: F, t10776: F, t3308: F, t9165: F, t9169: F, t12479: F, t37641: F, t10772: F, t9261: F, t9135: F, t9139: F, t9143: F, t3295: F, t9526: F, t27067: F, t3610: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t43034 = t37685 * t12476;
    let t43037 = t10776 * t3308 * t9165;
    let t43040 = t10776 * t3308 * t9169;
    let t43042 = t37641 * t12479;
    let t43045 = t10772 * t3308 * t9261;
    let t43048 = t10772 * t3308 * t9135;
    let t43051 = t10776 * t3308 * t9139;
    let t43054 = t10772 * t3308 * t9143;
    let t43057 = t3295 * t9526;
    let t43061 = t27067 * t3610;
    (t43034, t43037, t43040, t43042, t43045, t43048, t43051, t43054, t43057, t43061)
}

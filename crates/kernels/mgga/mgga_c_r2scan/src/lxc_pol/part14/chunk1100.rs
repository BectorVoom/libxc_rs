//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1100/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1100<F: Float>(t3270: F, t39014: F, t1114: F, t6897: F, t2330: F, t3492: F, t5086: F, t37358: F, t37386: F, t37397: F, t37406: F, t37412: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39015 = t3270 * t39014;
    let t39030 = t1114 * t6897;
    let t39032 = t3270 * t39030 * t2330;
    let t39040 = t5086 * t3492;
    let t39046 = F::new(0.26021382394247697185e-3) * t37358;
    let t39054 = F::new(0.205201155180140685e-5) * t37386;
    let t39059 = F::new(0.487802396665200453e-2) * t37397;
    let t39061 = F::new(0.11709622077411463733e-2) * t37406;
    let t39062 = F::new(0.18292589874945016987e-2) * t37412;
    (t39015, t39030, t39032, t39040, t39046, t39054, t39059, t39061, t39062)
}

//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1254/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1254<F: Float>(t11774: F, t654: F, t5273: F, t5531: F, t9758: F, t1907: F, t33066: F, t2041: F, t33302: F, t12351: F, t2811: F, t4816: F, t33018: F, t5074: F, t32998: F, t10872: F, t1791: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t112102 = t11774 * t654;
    let t112121 = t5273 * t654;
    let t112139 = t9758 * t5531;
    let t112149 = t33066 * t1907;
    let t112167 = t33302 * t2041;
    let t112173 = t2811 * t12351;
    let t112176 = t4816 * t654;
    let t112180 = t5074 * t33018;
    let t112182 = t5074 * t32998;
    let t112184 = t10872 * t1791;
    (t112102, t112121, t112139, t112149, t112167, t112173, t112176, t112180, t112182, t112184)
}

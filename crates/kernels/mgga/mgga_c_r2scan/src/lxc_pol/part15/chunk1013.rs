//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1013/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1013<F: Float>(t10760: F, t24714: F, t6535: F, t3295: F, t7520: F, t3308: F, t6362: F, t8030: F, t2834: F, t3344: F, t3290: F, t7301: F, t3591: F, t37972: F, t10872: F, t11736: F) -> (F, F, F, F, F, F, F) {
    let t39540 = t6535 * t10760 * t24714;
    let t39542 = t3295 * t7520;
    let t39545 = t6362 * t3308 * t8030;
    let t39548 = t2834 * t3344;
    let t39549 = 0.47609969197673950972e-2 * t39548;
    let t39550 = t3290 * t7301;
    let t39552 = t37972 * t3591;
    let t39554 = t10872 * t11736;
    (t39540, t39542, t39545, t39549, t39550, t39552, t39554)
}

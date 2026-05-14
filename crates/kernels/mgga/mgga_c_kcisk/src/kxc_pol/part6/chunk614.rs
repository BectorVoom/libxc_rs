//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 614/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk614<F: Float>(t2014: F, t7718: F, t1775: F, t5486: F, t7715: F, t5006: F, t2642: F, t5508: F, t1586: F, t20: F, t8857: F, t780: F, t2629: F, t2633: F, t41: F, t8616: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9168 = t2014 * t7718;
    let t9169 = t1775 * t9168;
    let t9172 = t5486 * t7715;
    let t9173 = t5006 * t9172;
    let t9176 = t2642 * t2642;
    let t9177 = t5508 * t9176;
    let t9178 = t1586 * t9177;
    let t9183 = t8857 * t20;
    let t9184 = t780 * t9183;
    let t9189 = t2629 * t2633;
    let t9192 = t8616 * t41;
    (t9168, t9169, t9172, t9173, t9176, t9177, t9178, t9183, t9184, t9189, t9192)
}

//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 651/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk651<F: Float>(t317: F, t337: F, t280: F, t1632: F, t1625: F, t4764: F, t5039: F, t5045: F, t5068: F, t4944: F, t4980: F, t4996: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5686 = t317 * t337;
    let t5687 = t5686 * t280;
    let t5688 = t1632 * t5687;
    let t5689 = t5688 * t1625;
    let t5691 = F::cast_from(0.06655833038988691_f64) * t4764;
    let t5693 = F::cast_from(0.10237773105191754_f64) * t5039;
    let t5694 = F::cast_from(0.06825182070127836_f64) * t5045;
    let t5696 = F::cast_from(0.02275060690042612_f64) * t5068;
    let t5701 = F::cast_from(0.04933718966136796_f64) * t4944;
    let t5703 = F::cast_from(0.14975624337724558_f64) * t4980;
    let t5706 = F::cast_from(0.1110086767380779_f64) * t4996;
    (t5687, t5689, t5691, t5693, t5694, t5696, t5701, t5703, t5706)
}

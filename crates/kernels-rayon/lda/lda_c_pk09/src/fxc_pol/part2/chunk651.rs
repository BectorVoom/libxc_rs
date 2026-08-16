//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 651/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk651(t317: f64, t337: f64, t280: f64, t1632: f64, t1625: f64, t4764: f64, t5039: f64, t5045: f64, t5068: f64, t4944: f64, t4980: f64, t4996: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5686 = t317 * t337;
    let t5687 = t5686 * t280;
    let t5688 = t1632 * t5687;
    let t5689 = t5688 * t1625;
    let t5691 = 0.06655833038988691_f64 * t4764;
    let t5693 = 0.10237773105191754_f64 * t5039;
    let t5694 = 0.06825182070127836_f64 * t5045;
    let t5696 = 0.02275060690042612_f64 * t5068;
    let t5701 = 0.04933718966136796_f64 * t4944;
    let t5703 = 0.14975624337724558_f64 * t4980;
    let t5706 = 0.1110086767380779_f64 * t4996;
    (t5687, t5689, t5691, t5693, t5694, t5696, t5701, t5703, t5706)
}

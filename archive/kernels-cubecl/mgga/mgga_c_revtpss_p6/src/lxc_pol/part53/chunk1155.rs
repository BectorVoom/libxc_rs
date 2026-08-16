//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1155/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1155<F: Float>(t32195: F, t32206: F, t5627: F, t9955: F, t125587: F, t32211: F, t3936: F, t13975: F, t246: F, t32289: F, t8591: F, t121031: F, t121186: F, t121199: F, t121203: F, t121207: F, t125767: F, t125771: F, t125775: F, t125780: F, t125782: F, t125785: F, t125793: F, t125797: F, t125799: F, t27896: F, t28012: F, t32226: F, t32250: F, t5774: F, t8578: F, t8706: F) -> F {
    let t125803 = t32206 * t9955 * t32195 * t5627;
    let t125807 = t32206 * t3936 * t32211 * t125587;
    let t125814 = t8591 * t32289 * t246 * t13975;
    let t125816 = -F::cast_from(0.17354086964223805049e-2_f64) * t125767 - F::cast_from(0.28234466758480466999e-3_f64) * t125771 + F::cast_from(0.34694512752820797848e1_f64) * t121031 * t27896 + F::cast_from(0.3718732920905101082e-3_f64) * t125775 + F::cast_from(0.25389723392137995738e-1_f64) * t121186 + F::cast_from(0.112937867033921868e-2_f64) * t125780 + F::cast_from(0.131760844872908846e-2_f64) * t125782 + F::cast_from(0.3718732920905101082e-3_f64) * t125785 - F::cast_from(0.17135921299530705785e1_f64) * t8706 * t32250 * t8578 * t5774 - F::cast_from(0.28234466758480466999e-3_f64) * t125793 + F::cast_from(0.18822977838986977999e-4_f64) * t125797 - F::cast_from(0.33467254597718846885e-4_f64) * t125799 + F::cast_from(0.56468933516960934e-2_f64) * t125803 + F::cast_from(0.112937867033921868e-2_f64) * t125807 + t121199 + t121203 - F::cast_from(0.13386901839087538754e-3_f64) * t121207 - F::cast_from(0.17347256376410398924e1_f64) * t32226 * t28012 + F::cast_from(0.7437465841810202164e-3_f64) * t125814;
    t125816
}

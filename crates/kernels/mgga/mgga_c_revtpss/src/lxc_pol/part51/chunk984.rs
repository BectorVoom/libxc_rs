//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 984/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk984<F: Float>(t121031: F, t121186: F, t121199: F, t121203: F, t121207: F, t125767: F, t125771: F, t125775: F, t125780: F, t125782: F, t125785: F, t125793: F, t125797: F, t125799: F, t125803: F, t125807: F, t125814: F, t27896: F, t28012: F, t32226: F, t32250: F, t5774: F, t8578: F, t8706: F) -> (F,) {
    let t125816 = -0.17354086964223805049e-2 * t125767 - 0.28234466758480466999e-3 * t125771 + 0.34694512752820797848e1 * t121031 * t27896 + 0.3718732920905101082e-3 * t125775 + 0.25389723392137995738e-1 * t121186 + 0.112937867033921868e-2 * t125780 + 0.131760844872908846e-2 * t125782 + 0.3718732920905101082e-3 * t125785 - 0.17135921299530705785e1 * t8706 * t32250 * t8578 * t5774 - 0.28234466758480466999e-3 * t125793 + 0.18822977838986977999e-4 * t125797 - 0.33467254597718846885e-4 * t125799 + 0.56468933516960934e-2 * t125803 + 0.112937867033921868e-2 * t125807 + t121199 + t121203 - 0.13386901839087538754e-3 * t121207 - 0.17347256376410398924e1 * t32226 * t28012 + 0.7437465841810202164e-3 * t125814;
    (t125816,)
}

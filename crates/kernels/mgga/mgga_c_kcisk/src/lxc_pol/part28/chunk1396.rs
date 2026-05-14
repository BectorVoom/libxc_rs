//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1396/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1396<F: Float>(t121037: F, t121074: F, t121107: F, t121152: F, t121183: F, t121216: F, t121267: F, t121309: F, t121335: F, t121371: F, t121411: F, t121435: F, t121476: F, t121516: F, t121553: F, t121578: F, t121619: F, t121657: F, t121695: F, t121726: F, t121762: F, t121791: F, t121826: F, t121866: F, t121905: F, t121930: F, t121953: F, t121984: F, t122020: F, t122052: F, t122096: F, t122133: F, t752: F) -> (F,) {
    let t122139 = (t121619 + t121152 + t121726 + t121984 + t121553 + t121476 + t121267 + t121435 + t122096 + t121107 + t121516 + t121930 + t121335 + t121037 + t121074 + t121657 + t122133 + t121695 + t121216 + t121866 + t121371 + t121826 + t121905 + t122020 + t121578 + t121183 + t121953 + t121791 + t122052 + t121411 + t121762 + t121309) * t752;
    (t122139,)
}

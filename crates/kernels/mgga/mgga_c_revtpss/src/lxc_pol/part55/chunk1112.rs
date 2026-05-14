//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1112/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1112<F: Float>(t128843: F, t32705: F, t32710: F, t32275: F, t32707: F, t98040: F, t122335: F, t27989: F, t122357: F, t125923: F, t125930: F, t121356: F, t122512: F, t125925: F, t125928: F, t3140: F, t5710: F, t8477: F, t8709: F) -> (F,) {
    let t128844 = t32705 * t128843;
    let t128846 = t32710 * t128843;
    let t128850 = t98040 * t32275 * t32707;
    let t128852 = t122335 * t27989;
    let t128854 = t122357 * t27989;
    let t128856 = 0.14874931683620404328e-3 * t125923;
    let t128859 = 0.17354086964223805049e-2 * t125930;
    let t128860 = 0.57119737665102352616e0 * t8477 * t5710 * t3140 * t8709 - 0.14279934416275588154e-1 * t128844 + 0.25389723392137995738e-1 * t128846 - 0.69416347856895220196e-2 * t121356 - 0.14279934416275588154e-1 * t128850 - 0.14456046980341999104e-1 * t128852 + 0.25702851531048074406e-1 * t128854 + t122512 - t128856 + 0.26447628533477078895e-3 * t125925 - 0.3718732920905101082e-3 * t125928 + t128859;
    (t128860,)
}

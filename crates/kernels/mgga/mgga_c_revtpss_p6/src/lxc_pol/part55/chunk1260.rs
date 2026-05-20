//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1260/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1260<F: Float>(t34226: F, t686: F, t72: F, t32705: F, t32710: F, t32275: F, t32707: F, t98040: F, t122335: F, t27989: F, t122357: F, t125923: F) -> (F, F, F, F, F, F) {
    let t128843 = t34226 * t72 * t686;
    let t128844 = t32705 * t128843;
    let t128846 = t32710 * t128843;
    let t128850 = t98040 * t32275 * t32707;
    let t128852 = t122335 * t27989;
    let t128854 = t122357 * t27989;
    let t128856 = F::cast_from(0.14874931683620404328e-3_f64) * t125923;
    (t128844, t128846, t128850, t128852, t128854, t128856)
}

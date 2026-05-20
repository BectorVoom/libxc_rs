//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3407/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3407<F: Float>(t19133: F, t2989: F, t981: F, t15559: F, t4719: F, t11591: F, t6223: F, t19049: F, t3026: F, t15556: F, t19146: F, t3007: F) -> (F, F, F, F, F, F) {
    let t63923 = F::cast_from(0.14035736694323150897e2_f64) * t981 * t19133 * t2989;
    let t63925 = F::cast_from(0.70178683471615754484e1_f64) * t4719 * t15559;
    let t63927 = F::cast_from(0.5848223622634646207e0_f64) * t11591 * t6223;
    let t63929 = F::cast_from(0.11696447245269292414e1_f64) * t19049 * t3026;
    let t63934 = F::cast_from(0.34631718211362927517e2_f64) * t4719 * t15556;
    let t63937 = F::cast_from(0.11696447245269292414e1_f64) * t981 * t19146 * t3007;
    (t63923, t63925, t63927, t63929, t63934, t63937)
}

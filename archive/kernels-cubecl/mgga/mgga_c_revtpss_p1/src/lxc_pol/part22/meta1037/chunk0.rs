//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3626/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3626<F: Float>(t20382: F, t3520: F, t1196: F, t5206: F, t12500: F, t20895: F, t5205: F, t58000: F, t1757: F, t58708: F, t68605: F, t16662: F, t57818: F) -> (F, F, F, F, F) {
    let t68680 = t3520 * t20382;
    let t68683 = F::cast_from(0.34631718211362927518e2_f64) * t1196 * t68680 * t5206;
    let t68686 = F::cast_from(0.17315859105681463759e2_f64) * t1196 * t20895 * t12500;
    let t68689 = F::cast_from(0.34631718211362927518e2_f64) * t1196 * t5205 * t58000;
    let t68692 = F::cast_from(0.14035736694323150897e2_f64) * t58708 * t1757 * t68605;
    let t68694 = F::cast_from(0.38596750796862084161e3_f64) * t57818 * t16662;
    (t68683, t68686, t68689, t68692, t68694)
}

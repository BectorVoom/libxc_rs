//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1302/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1302<F: Float>(t13296: F, t2842: F, t7718: F, t1020: F, t4801: F, t92917: F, t13280: F, t26760: F, t26685: F, t26728: F, t27915: F, t7703: F, t93425: F, t93592: F, t95590: F, t95785: F, t95923: F, t95928: F, t95931: F, t95938: F) -> (F, F, F, F) {
    let t95946 = t2842 * t7718 * t13296;
    let t95949 = t1020 * t92917 * t4801;
    let t95952 = t1020 * t26760 * t13280;
    let t95954 = -F::new(0.92673611111111111113e-3) * t93592 * t95923 - F::new(0.44218518518518518517e-2) * t95928 + F::new(0.3684876543209876543e-2) * t95931 - F::new(0.18550940104166666667e-3) * t93425 * t95923 + F::new(0.18550940104166666667e-3) * t26728 * t27915 + t95938 - F::new(0.61836467013888888888e-4) * t26685 * t95785 + F::new(0.556528203125e-3) * t26685 * t95590 + F::new(0.27802083333333333334e-2) * t7703 * t95590 + F::new(0.18424382716049382715e-2) * t95946 - F::new(0.33163888888888888888e-2) * t95949 - F::new(0.16581944444444444444e-2) * t95952;
    (t95946, t95949, t95952, t95954)
}

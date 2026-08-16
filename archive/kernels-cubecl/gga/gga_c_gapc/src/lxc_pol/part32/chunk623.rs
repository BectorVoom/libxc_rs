//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 623/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk623<F: Float>(t1743: F, t3717: F, t1912: F, t3666: F, t3671: F, t3676: F, t3681: F, t3685: F, t3689: F, t3692: F, t3704: F, t3710: F, t3715: F) -> (F, F) {
    let t3718 = t1743 * t3717;
    let t3719 = t3718 * t1912;
    let t3721 = F::cast_from(0.20241536458333333334e-4_f64) * t3666 - F::cast_from(0.17376185052903442709e-3_f64) * t3671 - F::cast_from(0.12650960286458333334e-5_f64) * t3676 + F::cast_from(0.10860115658064651693e-4_f64) * t3681 - F::cast_from(0.11594181388521408695e-4_f64) * t3685 - F::cast_from(0.33765185592488808582e-6_f64) * t3689 + F::cast_from(0.28985453471303521737e-5_f64) * t3692 - F::cast_from(0.24583187891642252608e-8_f64) * t3704 + F::cast_from(0.10551620497652752682e-7_f64) * t3710 + F::cast_from(0.33148893438893365995e-7_f64) * t3715 - F::cast_from(0.45289771048911752714e-7_f64) * t3719;
    (t3718, t3721)
}

//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 601/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk601<F: Float>(t3784: F, t3789: F, t2660: F, t3717: F, t2767: F, t3753: F, t3758: F, t3763: F, t3766: F, t3770: F, t3773: F, t3776: F, t3782: F, t3785: F) -> (F, F) {
    let t3790 = t3784 * t3789;
    let t3792 = t2660 * t3717;
    let t3793 = t3792 * t2767;
    let t3795 = 0.10120442708333333334e-4 * t3753 - 0.17376185052903442709e-3 * t3758 - 0.63252766927083333336e-6 * t3763 + 0.10860115658064651693e-4 * t3766 - 0.11594181388521408695e-4 * t3770 - 0.16882049790461501058e-6 * t3773 + 0.28985453471303521737e-5 * t3776 - 0.61454016367594401047e-9 * t3782 + 0.52756405595192190805e-8 * t3785 + 0.16573913624765925007e-7 * t3790 - 0.45289771048911752714e-7 * t3793;
    (t3792, t3795)
}

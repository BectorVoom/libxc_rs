//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 641/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk641(t3784: f64, t3789: f64, t2660: f64, t3717: f64, t2767: f64, t3753: f64, t3758: f64, t3763: f64, t3766: f64, t3770: f64, t3773: f64, t3776: f64, t3782: f64, t3785: f64) -> (f64, f64) {
    let t3790 = t3784 * t3789;
    let t3792 = t2660 * t3717;
    let t3793 = t3792 * t2767;
    let t3795 = 0.10120442708333333334e-4_f64 * t3753 - 0.17376185052903442709e-3_f64 * t3758 - 0.63252766927083333336e-6_f64 * t3763 + 0.10860115658064651693e-4_f64 * t3766 - 0.11594181388521408695e-4_f64 * t3770 - 0.16882049790461501058e-6_f64 * t3773 + 0.28985453471303521737e-5_f64 * t3776 - 0.61454016367594401047e-9_f64 * t3782 + 0.52756405595192190805e-8_f64 * t3785 + 0.16573913624765925007e-7_f64 * t3790 - 0.45289771048911752714e-7_f64 * t3793;
    (t3792, t3795)
}

//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1384/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1384(t33956: f64, t33962: f64, t33967: f64, t33972: f64, t33975: f64, t33978: f64, t33980: f64, t33983: f64, t33988: f64, t33991: f64, t33998: f64, t34001: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36773 = 0.67402122125348062552e-7_f64 * t33956;
    let t36774 = 0.20041830772435757309e-6_f64 * t33962;
    let t36775 = 0.83645744500336823644e-8_f64 * t33967;
    let t36777 = 0.2318836277704281739e-4_f64 * t33972;
    let t36778 = 0.71696352428860134552e-9_f64 * t33975;
    let t36779 = 0.47797568285906756368e-9_f64 * t33978;
    let t36780 = 0.11594181388521408695e-4_f64 * t33980;
    let t36781 = 0.27312896163375289353e-9_f64 * t33983;
    let t36782 = 0.49755503537412447748e-6_f64 * t33988;
    let t36783 = 0.18310351929594268994e-5_f64 * t33991;
    let t36788 = 0.71158605186385727883e-8_f64 * t33998;
    let t36789 = 0.13493923611111111112e-4_f64 * t34001;
    (t36773, t36774, t36775, t36777, t36778, t36779, t36780, t36781, t36782, t36783, t36788, t36789)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1055/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1055(t2013: f64, t22889: f64, t623: f64, t6944: f64, t6947: f64, t2086: f64, t56: f64, t111: f64, t2011: f64, t2021: f64, t21868: f64, t22052: f64, t22752: f64, t22844: f64, t22851: f64, t22856: f64, t22859: f64, t22862: f64, t22865: f64, t22868: f64, t22872: f64, t22876: f64, t22880: f64, t22884: f64, t22887: f64, t5: f64, t628: f64, t629: f64, t636: f64) -> (f64, f64) {
    let t22890 = t22889 * t2013;
    let t22892 = t623 * t6944;
    let t22893 = t22892 * t6947;
    let t22895 = t56 * t2086;
    let t22896 = t111 * t22895;
    let t22905 = 7.0_f64 / 36.0_f64 * t22844 - t628 * t629 * t5 * t22052 / 48.0_f64 + 455.0_f64 / 162.0_f64 * t22851 + t22856 - 0.16299677793353920978e-1_f64 * t636 * t22859 - 0.30426065214260652492e0_f64 * t22862 - 0.13039742234683136782e0_f64 * t2021 * t22865 + 0.34482873909495406156e1_f64 * t22868 + 0.43465807448943789272e-1_f64 * t636 * t22872 + 0.65198711173415683908e-1_f64 * t636 * t22876 + 0.43465807448943789272e-1_f64 * t636 * t22880 + 0.97798066760123525863e-1_f64 * t2021 * t22884 - 35.0_f64 / 36.0_f64 * t22887 + 35.0_f64 / 12.0_f64 * t22890 + 7.0_f64 / 3.0_f64 * t22893 + 5.0_f64 / 4.0_f64 * t22896 * t629 * t5 * t22752 + 3.0_f64 / 16.0_f64 * t2011 * t629 * t5 * t21868;
    (t22895, t22905)
}

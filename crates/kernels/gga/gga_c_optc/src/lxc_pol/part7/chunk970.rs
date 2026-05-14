//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 970/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk970<F: Float>(t2003: F, t2010: F, t2013: F, t623: F, t6944: F, t6947: F, t2086: F, t56: F, t111: F, t2011: F, t2021: F, t21868: F, t22052: F, t22752: F, t22844: F, t22851: F, t22856: F, t22859: F, t22862: F, t22865: F, t22868: F, t22872: F, t22876: F, t22880: F, t22884: F, t22887: F, t5: F, t628: F, t629: F, t636: F) -> (F, F) {
    let t22889 = t2003 * t2010;
    let t22890 = t22889 * t2013;
    let t22892 = t623 * t6944;
    let t22893 = t22892 * t6947;
    let t22895 = t56 * t2086;
    let t22896 = t111 * t22895;
    let t22905 = 7.0 / 36.0 * t22844 - t628 * t629 * t5 * t22052 / 48.0 + 455.0 / 162.0 * t22851 + t22856 - 0.16299677793353920978e-1 * t636 * t22859 - 0.30426065214260652492e0 * t22862 - 0.13039742234683136782e0 * t2021 * t22865 + 0.34482873909495406156e1 * t22868 + 0.43465807448943789272e-1 * t636 * t22872 + 0.65198711173415683908e-1 * t636 * t22876 + 0.43465807448943789272e-1 * t636 * t22880 + 0.97798066760123525863e-1 * t2021 * t22884 - 35.0 / 36.0 * t22887 + 35.0 / 12.0 * t22890 + 7.0 / 3.0 * t22893 + 5.0 / 4.0 * t22896 * t629 * t5 * t22752 + 3.0 / 16.0 * t2011 * t629 * t5 * t21868;
    (t22895, t22905)
}

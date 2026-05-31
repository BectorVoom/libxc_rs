//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1387/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1387<F: Float>(t3931: F, t810: F, t3703: F, t944: F, t15392: F, t321: F, t1105: F, t1167: F, t11737: F, t13751: F, t13756: F, t14153: F, t14390: F, t14821: F, t2494: F, t3324: F, t3944: F, t3946: F, t4062: F, t4063: F, t52799: F, t52861: F, t52884: F, t52887: F, t56059: F, t9807: F) -> F {
    let t57820 = t3931 * t810;
    let t57830 = t3703 * t944;
    let t57840 = t321 * t15392;
    let t57853 = F::cast_from(6.0_f64) * t1105 * t3946 * t52799 - F::cast_from(2.0_f64) * t1167 * t4062 * t52861 + F::cast_from(6.0_f64) * t11737 * t13756 * t3944 + F::cast_from(6.0_f64) * t13751 * t13756 * t3703 - F::cast_from(6.0_f64) * t13756 * t4063 * t57830 + F::cast_from(6.0_f64) * t14153 * t3946 * t57820 + F::cast_from(6.0_f64) * t14390 * t2494 * t3946 - F::cast_from(2.0_f64) * t14821 * t3324 * t4062 + F::cast_from(3.0_f64) * t3944 * t3946 * t9807 + F::cast_from(3.0_f64) * t3946 * t56059 * t810 + t52884 + t52887 - t57840;
    t57853
}

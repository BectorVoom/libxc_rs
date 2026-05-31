//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1070/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1070<F: Float>(t35621: F, t8392: F, t10007: F, t10085: F, t1091: F, t110401: F, t13839: F, t13885: F, t14127: F, t14163: F, t141713: F, t141722: F, t141744: F, t141746: F, t141752: F, t141902: F, t141989: F, t150120: F, t151461: F, t151471: F, t1901: F, t24737: F, t2599: F, t27983: F, t28108: F, t28140: F, t28299: F, t28300: F, t28364: F, t33476: F, t33715: F, t33760: F, t35553: F, t35562: F, t3837: F, t3842: F, t3859: F, t3864: F, t3880: F, t684: F) -> F {
    let t151483 = t8392 * t35621;
    let t151493 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t141722 - F::cast_from(2.0_f64) * t1901 * t28140 * t33715 * t3837 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t13885 * t141989 * t3842 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t13839 * t33760 + t1901 * t10085 * t35562 / F::cast_from(9.0_f64) + t1901 * t2599 * t141902 * t1091 / F::cast_from(9.0_f64) + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t141744 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t141746 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t10007 * t35553 * t684 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t13885 * t24737 * t28108 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t151461 - F::cast_from(4.0_f64) * t1901 * t28299 * t28300 * t27983 + F::cast_from(2.0_f64) * t1901 * t28140 * t141713 * t3859 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t14127 * t151471 * t3864 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t13885 * t141989 * t3859 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t110401 * t28364 - t151483 / F::cast_from(27.0_f64) - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t141752 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t14163 * t150120 - t1901 * t10007 * t33476 * t3880 / F::cast_from(9.0_f64);
    t151493
}

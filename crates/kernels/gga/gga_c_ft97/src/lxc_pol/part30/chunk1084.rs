//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1084/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1084<F: Float>(t35614: F, t8392: F, t10079: F, t1091: F, t109848: F, t110629: F, t11593: F, t124402: F, t13885: F, t14127: F, t14187: F, t142333: F, t142334: F, t142365: F, t142382: F, t142393: F, t142395: F, t149867: F, t150042: F, t151353: F, t151430: F, t1901: F, t2347: F, t242: F, t2469: F, t33532: F, t33771: F, t33772: F, t35634: F, t3746: F, t3859: F, t3864: F, t3886: F, t446: F, t52006: F, t53662: F, t6166: F, t6175: F, t67996: F, t724: F, t729: F, t7502: F, t7553: F) -> F {
    let t152361 = t8392 * t35614;
    let t152404 = t446 * t729 * t2469 * t35634 / F::cast_from(3.0_f64) - t446 * t242 * t151353 / F::cast_from(3.0_f64) + t142333 - t446 * t724 * t33532 * t1091 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t142334 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t152361 - t142365 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t142382 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11593 * t10079 * t33771 * t3746 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t52006 * t33772 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t13885 * t110629 * t6166 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t14127 * t124402 * t6175 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t53662 * t150042 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t142393 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t142395 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t242 * t151430 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t242 * t149867 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t67996 * t7502 * t3859 + F::cast_from(4.0_f64) * t1901 * t109848 * t7502 * t3864 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t14187 * t7553 * t2347 * t3886;
    t152404
}

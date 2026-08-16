//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 939/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk939<F: Float>(t36512: F, t41339: F, t10742: F, t10867: F, t900: F, t44130: F, t13658: F, t2197: F, t1036: F, t11001: F, t13661: F, t1445: F, t3050: F, t33725: F, t33778: F, t44124: F, t44128: F, t44133: F, t44147: F, t44157: F, t44771: F, t45922: F, t45931: F, t45933: F, t45939: F, t45942: F, t45946: F, t4614: F, t723: F, t780: F, t813: F) -> F {
    let t45947 = t36512 * t41339;
    let t45950 = t10867 * t900 * t10742;
    let t45953 = F::cast_from(0.17875244975925213335e0_f64) * t44130;
    let t45958 = F::cast_from(0.43710935587469654631e2_f64) * t2197 * t13658;
    let t45959 = F::cast_from(0.17041300423964777634e0_f64) * t44124 - t45922 - F::cast_from(0.12269736305254639897e2_f64) * t813 * t4614 * t13661 - F::cast_from(0.46011511144704899612e1_f64) * t813 * t1445 * t44771 * t723 + t45931 - t45933 + F::cast_from(0.71500979903700853338e0_f64) * t1036 * t33725 + F::cast_from(0.47667319935800568892e0_f64) * t1036 * t33778 + t45939 + F::cast_from(0.35750489951850426669e0_f64) * t780 * t45942 - t45946 + F::cast_from(0.44688112439813033338e-1_f64) * t45947 - F::cast_from(0.89376224879626066676e-1_f64) * t45950 - F::cast_from(0.17041300423964777634e0_f64) * t44128 - t45953 + F::cast_from(0.63904876589867916128e-1_f64) * t44133 + F::cast_from(0.71500979903700853338e0_f64) * t3050 * t11001 + t44147 - t44157 + t45958;
    t45959
}

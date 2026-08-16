//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 939/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk939(t36512: f64, t41339: f64, t10742: f64, t10867: f64, t900: f64, t44130: f64, t13658: f64, t2197: f64, t1036: f64, t11001: f64, t13661: f64, t1445: f64, t3050: f64, t33725: f64, t33778: f64, t44124: f64, t44128: f64, t44133: f64, t44147: f64, t44157: f64, t44771: f64, t45922: f64, t45931: f64, t45933: f64, t45939: f64, t45942: f64, t45946: f64, t4614: f64, t723: f64, t780: f64, t813: f64) -> f64 {
    let t45947 = t36512 * t41339;
    let t45950 = t10867 * t900 * t10742;
    let t45953 = 0.17875244975925213335e0_f64 * t44130;
    let t45958 = 0.43710935587469654631e2_f64 * t2197 * t13658;
    let t45959 = 0.17041300423964777634e0_f64 * t44124 - t45922 - 0.12269736305254639897e2_f64 * t813 * t4614 * t13661 - 0.46011511144704899612e1_f64 * t813 * t1445 * t44771 * t723 + t45931 - t45933 + 0.71500979903700853338e0_f64 * t1036 * t33725 + 0.47667319935800568892e0_f64 * t1036 * t33778 + t45939 + 0.35750489951850426669e0_f64 * t780 * t45942 - t45946 + 0.44688112439813033338e-1_f64 * t45947 - 0.89376224879626066676e-1_f64 * t45950 - 0.17041300423964777634e0_f64 * t44128 - t45953 + 0.63904876589867916128e-1_f64 * t44133 + 0.71500979903700853338e0_f64 * t3050 * t11001 + t44147 - t44157 + t45958;
    t45959
}

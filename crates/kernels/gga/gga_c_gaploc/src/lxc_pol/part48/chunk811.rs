//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 811/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk811<F: Float>(t44130: F, t13658: F, t2197: F, t1036: F, t11001: F, t13661: F, t1445: F, t3050: F, t33725: F, t33778: F, t44124: F, t44128: F, t44133: F, t44147: F, t44157: F, t44771: F, t45922: F, t45931: F, t45933: F, t45939: F, t45942: F, t45946: F, t45947: F, t45950: F, t4614: F, t723: F, t780: F, t813: F) -> (F,) {
    let t45953 = 0.17875244975925213335e0 * t44130;
    let t45958 = 0.43710935587469654631e2 * t2197 * t13658;
    let t45959 = 0.17041300423964777634e0 * t44124 - t45922 - 0.12269736305254639897e2 * t813 * t4614 * t13661 - 0.46011511144704899612e1 * t813 * t1445 * t44771 * t723 + t45931 - t45933 + 0.71500979903700853338e0 * t1036 * t33725 + 0.47667319935800568892e0 * t1036 * t33778 + t45939 + 0.35750489951850426669e0 * t780 * t45942 - t45946 + 0.44688112439813033338e-1 * t45947 - 0.89376224879626066676e-1 * t45950 - 0.17041300423964777634e0 * t44128 - t45953 + 0.63904876589867916128e-1 * t44133 + 0.71500979903700853338e0 * t3050 * t11001 + t44147 - t44157 + t45958;
    (t45959,)
}

//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1362/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1362<F: Float>(t10012: F, t116340: F, t116344: F, t117857: F, t117868: F, t117873: F, t117876: F, t117880: F, t117887: F, t18377: F, t18382: F, t33196: F, t33208: F, t33219: F, t33220: F, t33297: F, t34552: F, t4648: F, t9740: F) -> (F,) {
    let t117889 = 0.34722222222222222222e-2 * t33208 * t34552 + 0.17361111111111111111e-2 * t9740 * t33219 * t33220 * t18377 - 0.69444444444444444444e-2 * t9740 * t117857 * t33220 * t18382 + 0.17361111111111111111e-2 * t9740 * t33219 * t10012 * t4648 + 0.20833333333333333334e-1 * t9740 * t117868 + t117873 + t117876 - t117880 + 0.34722222222222222222e-2 * t33297 * t34552 + 0.120625e-1 * t33196 * t117868 - 0.23214722222222222222e-2 * t116340 - 0.11607361111111111111e-2 * t116344 + 0.11574074074074074074e-2 * t117887;
    (t117889,)
}

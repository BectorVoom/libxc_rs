//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1435/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1435<F: Float>(t34465: F, t9991: F, t10005: F, t10014: F, t116998: F, t117752: F, t117824: F, t118343: F, t121748: F, t121751: F, t121754: F, t121757: F, t121760: F, t34462: F, t34520: F, t34580: F, t35427: F, t9728: F, t9748: F, t9995: F) -> (F,) {
    let t123030 = t9991 * t34465;
    let t123044 = -0.27777777777777777778e-1 * t34462 * t9995 - 0.27777777777777777778e-1 * t10005 * t34520 - 0.10722222222222222222e-1 * t117752 * t9995 - 0.27777777777777777778e-1 * t34580 * t10014 + 0.34722222222222222223e-2 * t123030 - 0.23214722222222222222e-2 * t121748 + 0.10416666666666666667e-1 * t35427 * t9748 - 0.10722222222222222222e-1 * t117824 * t9995 - 0.23214722222222222222e-2 * t121751 - 0.23214722222222222222e-2 * t121754 + 0.46429444444444444444e-2 * t121757 + 0.10416666666666666667e-1 * t35427 * t9728 + 0.61905925925925925925e-2 * t121760 - t118343 + 0.61905925925925925925e-2 * t116998;
    (t123044,)
}

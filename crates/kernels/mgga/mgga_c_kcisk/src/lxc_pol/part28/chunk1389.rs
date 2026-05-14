//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1389/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1389<F: Float>(t116960: F, t116983: F, t117153: F, t117159: F, t117161: F, t117170: F, t121071: F, t121960: F, t121971: F, t121973: F, t121976: F, t121979: F, t121982: F, t17345: F, t22294: F, t33031: F, t34013: F, t34032: F, t34037: F, t9649: F, t9922: F) -> (F,) {
    let t121984 = 0.20833333333333333334e-1 * t116983 * t9922 + 0.61728395061728395063e-2 * t117153 - t117159 + 0.80416666666666666669e-2 * t9649 * t121071 - t117161 + 0.23148148148148148149e-2 * t121960 - 0.27777777777777777778e-1 * t33031 * t17345 * t34037 * t22294 + 0.69444444444444444446e-2 * t116960 * t34032 + 0.69444444444444444446e-2 * t116960 * t34013 + 0.24320185185185185185e-1 * t121971 - t117170 + 0.14739506172839506172e-2 * t121973 - 0.36848765432098765431e-3 * t121976 + 0.66327777777777777776e-2 * t121979 + 0.11054629629629629629e-2 * t121982;
    (t121984,)
}

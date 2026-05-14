//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1158/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1158<F: Float>(t6854: F, t8232: F, t1882: F, t28426: F, t28417: F, t681: F, t89: F, t109620: F, t109623: F, t13839: F, t14108: F, t14127: F, t1901: F, t2354: F, t242: F, t24570: F, t24722: F, t24760: F, t24797: F, t2574: F, t265: F, t27767: F, t28098: F, t28128: F, t3281: F, t4005: F, t446: F, t52018: F, t53923: F, t6079: F, t6135: F, t67796: F, t684: F, t724: F, t97831: F) -> (F,) {
    let t110845 = t8232 * t6854;
    let t110859 = 2.0 / 9.0 * t1882 * t28426;
    let t110872 = 2.0 / 9.0 * t89 * t681 * t28417;
    let t110886 = -2.0 / 9.0 * t3281 * t2354 * t265 * t6135 + 8.0 / 27.0 * t110845 - 2.0 / 9.0 * t446 * t724 * t28098 * t684 + 4.0 / 3.0 * t446 * t2574 * t4005 * t6079 + 4.0 / 3.0 * t446 * t242 * t109620 - t110859 + 2.0 / 3.0 * t446 * t242 * t109623 - 2.0 / 9.0 * t1901 * t13839 * t24797 - 4.0 / 3.0 * t1901 * t14127 * t28128 * t14108 - t110872 + 2.0 / 27.0 * t97831 - 2.0 / 9.0 * t1901 * t53923 * t24570 - 4.0 / 9.0 * t1901 * t67796 * t24760 - 2.0 / 9.0 * t1901 * t53923 * t24722 - 4.0 / 9.0 * t1901 * t52018 * t27767;
    (t110886,)
}

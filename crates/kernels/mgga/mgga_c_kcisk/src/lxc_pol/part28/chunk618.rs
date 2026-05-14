//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 618/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk618<F: Float>(t2487: F, t682: F, t1648: F, t4629: F, t673: F, t707: F, t1824: F, t2488: F, t1887: F, t6790: F, t706: F, t1421: F, t1689: F, t2399: F, t456: F, t604: F, t6884: F, t7025: F, t7031: F, t7036: F, t7040: F, t7043: F, t7047: F) -> (F, F, F, F, F, F, F, F) {
    let t7050 = t682 * t2487;
    let t7051 = t7050 * t1648;
    let t7052 = t4629 * t7051;
    let t7055 = t673 * t707;
    let t7056 = t2488 * t1824;
    let t7057 = t7055 * t7056;
    let t7060 = t1887 * t6790;
    let t7061 = t706 * t7060;
    let t7068 = 0.98556445e-3 * t1421 * t7025 + 0.7391733375e-3 * t1421 * t7031 - 0.1478346675e-2 * t1421 * t7036 + 0.1478346675e-2 * t456 * t7040 - 0.65704296666666666667e-3 * t7043 - 0.65704296666666666667e-3 * t1421 * t7047 - 0.1478346675e-2 * t1421 * t7052 + 0.19711289e-2 * t1421 * t7057 - 0.98556445e-3 * t456 * t7061 - 4.0 * t1689 * t2399 - 4.0 * t604 * t6884;
    (t7051, t7052, t7055, t7056, t7057, t7060, t7061, t7068)
}

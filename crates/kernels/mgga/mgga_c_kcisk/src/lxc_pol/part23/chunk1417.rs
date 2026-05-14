//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1417/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1417<F: Float>(t13900: F, t9536: F, t9863: F, t12261: F, t2737: F, t9868: F, t32474: F, t33873: F, t33766: F, t9535: F, t109366: F, t109896: F, t109925: F, t113890: F, t32339: F, t32371: F, t32436: F, t32443: F, t33762: F, t33784: F, t33827: F, t33864: F, t9512: F, t9524: F, t9855: F, t9864: F, t9869: F) -> (F, F) {
    let t115337 = t9536 * t13900 * t9863;
    let t115346 = t2737 * t12261 * t9868;
    let t115351 = 0.13402777777777777778e-2 * t32474 * t33873;
    let t115358 = t33766 * t9535;
    let t115367 = -0.51588271604938271604e-3 * t113890 + 0.38580246913580246913e-3 * t115337 + 0.10416666666666666667e-1 * t9512 * t33864 + 0.52083333333333333333e-2 * t32371 * t9869 + 0.10416666666666666667e-1 * t9524 * t33864 - 0.11574074074074074074e-2 * t115346 - 0.10722222222222222222e-1 * t109896 * t9855 + t115351 - 0.17361111111111111111e-2 * t109366 * t9864 - 0.34722222222222222222e-2 * t109925 * t9864 - 0.69444444444444444444e-2 * t32436 * t33827 - 0.40208333333333333334e-2 * t115358 * t32443 + 0.27777777777777777778e-1 * t32339 * t33762 + 0.55555555555555555557e-1 * t32339 * t33784 - 0.10416666666666666667e-1 * t32436 * t33762;
    (t115358, t115367)
}

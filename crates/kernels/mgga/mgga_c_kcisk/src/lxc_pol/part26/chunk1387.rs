//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1387/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1387<F: Float>(t1596: F, t32440: F, t6204: F, t8398: F, t33873: F, t9860: F, t109846: F, t115085: F, t115358: F, t119302: F, t119305: F, t119308: F, t119311: F, t119313: F, t33762: F, t33771: F, t33784: F, t33794: F, t33906: F, t33911: F, t9536: F) -> (F, F) {
    let t120468 = t6204 * t32440 * t8398 * t1596;
    let t120475 = t9860 * t33873;
    let t120489 = -0.52083333333333333333e-2 * t9536 * t120468 - 0.40208333333333333334e-2 * t115358 * t33762 - 0.120625e-1 * t115358 * t33784 + 0.34722222222222222223e-2 * t120475 - 0.34822083333333333332e-2 * t119302 + 0.11607361111111111111e-2 * t119305 + 0.11607361111111111111e-2 * t119308 - 0.38691203703703703703e-3 * t119311 - 0.25794135802469135802e-3 * t119313 - 0.38691203703703703703e-3 * t109846 + 0.34722222222222222222e-2 * t33794 * t33906 + 0.34722222222222222222e-2 * t33794 * t33911 + 0.13402777777777777778e-2 * t115085 * t33771;
    (t120468, t120489)
}

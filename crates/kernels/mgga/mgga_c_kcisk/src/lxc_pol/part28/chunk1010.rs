//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1010/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1010<F: Float>(t22484: F, t7242: F, t22387: F, t7234: F, t17345: F, t22488: F, t17317: F, t1773: F, t23344: F, t23348: F, t23357: F, t23362: F, t23366: F, t23370: F, t4989: F, t5013: F, t7258: F, t8802: F) -> (F,) {
    let t23373 = t7242 * t22484;
    let t23376 = t7234 * t22387;
    let t23379 = t17345 * t22488;
    let t23382 = -0.31983624384315230601e-1 * t23344 + 0.10794473229706390328e0 * t1773 * t23348 - 0.35981577432354634426e-1 * t4989 * t8802 - 0.35981577432354634427e-1 * t17317 * t7258 + 0.35981577432354634427e-1 * t5013 * t23357 - 0.17990788716177317213e-1 * t5013 * t23362 - 0.35981577432354634426e-1 * t5013 * t23366 + 0.71963154864709268852e-1 * t5013 * t23370 + 0.10794473229706390328e0 * t5013 * t23373 - 0.1439263097294185377e0 * t5013 * t23376 + 0.1439263097294185377e0 * t5013 * t23379;
    (t23382,)
}

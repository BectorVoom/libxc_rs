//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1255/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1255<F: Float>(t26574: F, t2743: F, t5322: F, t8987: F, t7654: F, t7657: F, t26571: F, t1419: F, t3124: F, t1859: F, t3162: F, t5377: F, t3165: F, t410: F, t8940: F, t1745: F, t3142: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28499 = t2743 * t26574;
    let t28503 = t8987 * t5322;
    let t28505 = t7657 * t7654;
    let t28507 = t2743 * t26571;
    let t28510 = t1419 * t3124;
    let t28522 = t1859 * t3162 * t5377;
    let t28525 = t1859 * t3165 * t5377;
    let t28553 = t410 * t8940;
    let t28555 = t3142 * t1745;
    (t28499, t28503, t28505, t28507, t28510, t28522, t28525, t28553, t28555)
}

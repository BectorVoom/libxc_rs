//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 710/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk710<F: Float>(t12670: F, t2610: F, t3720: F, t2365: F, t2033: F, t12252: F, t959: F, t12693: F, t12706: F, t13861: F, t1457: F, t2103: F, t13066: F, t13070: F, t13074: F, t13079: F, t13114: F, t13115: F, t13116: F, t13120: F) -> (F, F, F, F) {
    let t13890 = 0.38342925953920749677e0 * t12670;
    let t13891 = t2610 * t3720;
    let t13892 = t2365 * t13891;
    let t13893 = t2033 * t13892;
    let t13895 = t12252 * t959;
    let t13898 = 0.63904876589867916128e-1 * t12693;
    let t13899 = 0.63904876589867916128e-1 * t12706;
    let t13900 = t1457 * t13861;
    let t13901 = t2103 * t13900;
    let t13903 = -0.19171462976960374838e0 * t13066 - t13890 - 0.14896037479937677779e-1 * t13893 + 0.14896037479937677779e-1 * t13895 + 0.19171462976960374838e0 * t13070 - t13074 + t13079 - t13898 - t13114 + t13115 + t13116 + t13899 + t13120 + 0.71500979903700853338e0 * t13901;
    (t13891, t13892, t13900, t13903)
}

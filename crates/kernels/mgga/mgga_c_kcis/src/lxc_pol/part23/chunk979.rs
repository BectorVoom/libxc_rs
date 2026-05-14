//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 979/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk979<F: Float>(t28313: F, t446: F, t1300: F, t8255: F, t7886: F, t8130: F, t1885: F, t8014: F, t1299: F, t2132: F, t2233: F, t27364: F, t8164: F, t1394: F, t167: F, t4163: F) -> (F, F, F, F, F, F, F, F) {
    let t28314 = t446 * t28313;
    let t28316 = t1300 * t8255;
    let t28317 = t446 * t28316;
    let t28320 = t8130 * t7886;
    let t28322 = t1885 * t8014;
    let t28323 = t446 * t28322;
    let t28325 = t1299 * t2132;
    let t28326 = t2233 * t28325;
    let t28328 = t27364 * t8164;
    let t28329 = t1394 * t28328;
    let t28331 = t4163 * t167;
    (t28314, t28317, t28320, t28323, t28326, t28328, t28329, t28331)
}

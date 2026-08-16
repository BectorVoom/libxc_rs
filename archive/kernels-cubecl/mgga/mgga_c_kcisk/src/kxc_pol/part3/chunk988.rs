//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 988/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk988<F: Float>(t13944: F, t6332: F, t6331: F, t1483: F, t4313: F, t1512: F, t4305: F, t493: F, t1517: F, t4301: F, t1493: F, t4297: F) -> (F, F, F, F, F) {
    let t14555 = t6332 * t13944;
    let t14556 = t6331 * t14555;
    let t14558 = t1483 * t4313;
    let t14560 = t1512 * t4305;
    let t14561 = t493 * t14560;
    let t14563 = t4301 * t1517;
    let t14565 = t4297 * t1493;
    (t14556, t14558, t14561, t14563, t14565)
}

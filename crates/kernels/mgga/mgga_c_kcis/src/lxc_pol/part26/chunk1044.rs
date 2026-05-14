//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1044/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1044<F: Float>(t2233: F, t29247: F, t8130: F, t8136: F, t2273: F, t6888: F, t1885: F, t8255: F, t446: F, t27453: F, t6281: F, t5709: F, t28499: F, t8164: F, t1394: F, t4163: F, t6284: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29248 = t2233 * t29247;
    let t29249 = t29248 / 8.0;
    let t29250 = t8130 * t8136;
    let t29251 = t29250 / 8.0;
    let t29252 = t6888 * t2273;
    let t29253 = t29252 / 8.0;
    let t29254 = t1885 * t8255;
    let t29255 = t446 * t29254;
    let t29256 = t29255 / 8.0;
    let t29258 = t27453 * t6281;
    let t29259 = t5709 * t29258;
    let t29266 = t28499 * t8164;
    let t29267 = t1394 * t29266;
    let t29269 = t4163 * t6284;
    (t29249, t29251, t29253, t29256, t29258, t29259, t29266, t29267, t29269)
}

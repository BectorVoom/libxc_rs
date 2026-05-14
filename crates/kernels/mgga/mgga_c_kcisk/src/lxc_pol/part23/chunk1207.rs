//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1207/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1207<F: Float>(t1328: F, t1390: F, t5626: F, t6183: F, t3532: F, t6175: F, t32008: F, t32043: F, t32055: F, t32087: F, t32102: F, t32189: F, t33346: F, t33360: F, t33384: F, t33389: F, t33393: F, t33400: F, t9446: F, t9449: F, t9796: F) -> (F, F, F, F, F, F, F) {
    let t33408 = t1328 * t1390;
    let t33409 = t33408 * t5626;
    let t33410 = t6183 * t33409;
    let t33415 = t1328 * t3532;
    let t33416 = t33415 * t5626;
    let t33417 = t6175 * t33416;
    let t33420 = -0.34722222222222222223e-2 * t33384 * t9449 - 0.23280625000000000001e-2 * t32102 * t33389 - 0.44218518518518518517e-2 * t33393 - 0.10722222222222222223e-1 * t32189 * t9796 - 0.11574074074074074074e-2 * t32043 - 0.10416666666666666667e-1 * t9446 * t33400 - 0.20833333333333333334e-1 * t9446 * t33389 - 0.34722222222222222223e-2 * t32055 + 0.10416666666666666667e-1 * t9446 * t33346 + 0.69444444444444444446e-2 * t32087 * t33410 + 0.13402777777777777778e-2 * t32008 * t33360 - 0.46296296296296296297e-2 * t32087 * t33417;
    (t33408, t33409, t33410, t33415, t33416, t33417, t33420)
}

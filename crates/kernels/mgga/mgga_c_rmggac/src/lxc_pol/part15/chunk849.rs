//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 849/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk849<F: Float>(t1743: F, t1970: F, t1971: F, t209: F, t476: F, t511: F, t11905: F, t2376: F, t2868: F, t40063: F, t40076: F, t45938: F, t45942: F, t45947: F, t45949: F, t45951: F, t45956: F, t45960: F, t45964: F, t45966: F, t45974: F, t45976: F, t6557: F, t7567: F, t884: F, t9025: F) -> (F,) {
    let t45982 = t1970 * t1971 * t511 * t1743 * t476 * t209;
    let t45989 = 0.25538759935978703638e-4 * t45938 - 0.25538759935978703638e-4 * t45942 - 0.23942587439980034662e-4 * t45947 + 0.25538759935978703638e-4 * t45949 - 0.76616279807936110914e-4 * t45951 - 0.25538759935978703638e-4 * t45956 + 0.76616279807936110914e-4 * t45960 - 0.10215503974391481455e-3 * t45964 - 0.85129199786595678796e-5 * t45966 - 0.11974241701863808564e0 * t11905 * t2376 - t40063 - t40076 - 0.12769379967989351819e-4 * t45974 + 0.25538759935978703638e-4 * t45976 + 0.12769379967989351819e-4 * t45982 - 0.23948483403727617128e0 * t884 * t7567 * t6557 - 0.11974241701863808564e0 * t2868 * t9025;
    (t45989,)
}

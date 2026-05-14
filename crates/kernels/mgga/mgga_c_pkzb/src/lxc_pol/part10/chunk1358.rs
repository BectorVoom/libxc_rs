//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1358/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1358<F: Float>(t8319: F, t8392: F, t10044: F, t8467: F, t10047: F, t8456: F, t10083: F, t23088: F, t23091: F, t2380: F, t2381: F, t27104: F, t27113: F, t3185: F, t3187: F, t3919: F, t406: F, t6417: F, t6483: F, t8346: F, t8428: F, t8430: F, t8435: F, t8436: F, t8460: F, t8464: F, t8475: F) -> (F,) {
    let t27151 = t8319 * t8392;
    let t27153 = t10044 * t8467;
    let t27155 = t10047 * t8456;
    let t27163 = -0.11433071498151929859e-2 * t23088 + 0.57165357490759649296e-3 * t23091 + 0.85748036236139473944e-3 * t3185 * t406 * t27104 * t3187 + 0.42874018118069736972e-3 * t3185 * t406 * t10083 * t6417 + 0.12862205435420921092e-2 * t8428 * t406 * t27113 * t8430 - 0.12862205435420921092e-2 * t8435 * t406 * t27113 * t8436 - 0.42874018118069736972e-3 * t2380 * t2381 * t3919 * t6483 + 0.45732285992607719436e-2 * t8319 * t8346 + 0.60976381323476959248e-2 * t27151 - 0.60976381323476959248e-2 * t27153 + 0.30488190661738479624e-2 * t27155 + 0.91464571985215438872e-2 * t8319 * t8460 + 0.45732285992607719436e-2 * t8319 * t8464 + 0.91464571985215438872e-2 * t8319 * t8475;
    (t27163,)
}

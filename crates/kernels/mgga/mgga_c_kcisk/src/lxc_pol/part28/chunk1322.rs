//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1322/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1322<F: Float>(t116477: F, t9664: F, t74846: F, t9663: F, t9648: F, t1755: F, t2788: F, t18325: F, t34121: F, t34191: F, t112585: F, t719: F, t1772: F, t648: F, t64905: F, t32955: F, t34122: F) -> (F, F, F, F, F, F, F, F, F) {
    let t116479 = 0.13888888888888888889e-1 * t9664 * t116477;
    let t116482 = t9663 * t74846;
    let t116489 = t9648 * t74846;
    let t116509 = t2788 * t1755;
    let t116513 = t34121 * t18325;
    let t116516 = t34191 * t18325;
    let t116536 = t112585 * t719;
    let t116552 = t64905 * t648 * t1772;
    let t116599 = t34122 * t32955;
    (t116479, t116482, t116489, t116509, t116513, t116516, t116536, t116552, t116599)
}

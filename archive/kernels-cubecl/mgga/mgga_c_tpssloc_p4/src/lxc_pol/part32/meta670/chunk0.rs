//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2103/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2103<F: Float>(t15437: F, t24728: F, t24732: F, t4965: F, t7344: F, t1184: F, t24682: F, t27607: F, t1209: F, t85821: F, t15743: F, t7345: F) -> (F, F, F, F, F, F) {
    let t95270 = t15437 * t24728;
    let t95273 = t15437 * t24732;
    let t95276 = t4965 * t7344;
    let t95303 = t24682 * t27607 * t1184;
    let t95304 = t85821 * t1209;
    let t95320 = F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t7345 * t15743;
    (t95270, t95273, t95276, t95303, t95304, t95320)
}

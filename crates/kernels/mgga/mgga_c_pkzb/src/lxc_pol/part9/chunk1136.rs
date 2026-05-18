//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1136/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1136<F: Float>(t19655: F, t19682: F, t98: F, t124: F, t1545: F, t2605: F, t1548: F, t16476: F, t16193: F, t16230: F, t16273: F, t16275: F, t16280: F, t16283: F, t16287: F, t16290: F, t16481: F, t16486: F, t16489: F, t192: F, t19621: F, t19624: F, t19626: F, t19628: F, t2718: F, t568: F, t6853: F) -> (F, F, F, F, F, F) {
    let t19684 = (t19655 + t19682) * t98;
    let t19686 = F::new(0.19751673498613801407e-1) * t19684 * t124;
    let t19687 = t1545 * t2605;
    let t19688 = F::new(36.0) * t19687;
    let t19690 = F::new(96.0) * t1548 * t2605;
    let t19691 = F::new(0.10526802520742363173e2) * t16476;
    let t19692 = F::new(18.0) * t192 * t2718 * t568 * t6853 - t16193 - t16230 - t16273 + t16275 - t16280 + t16283 + t16287 - t16290 + t16481 - t16486 - t16489 - t19621 + t19624 + t19626 + t19628 + t19686 + t19688 - t19690 + t19691;
    (t19684, t19686, t19688, t19690, t19691, t19692)
}

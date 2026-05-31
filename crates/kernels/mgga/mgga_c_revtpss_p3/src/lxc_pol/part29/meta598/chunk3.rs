//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2030/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2030<F: Float>(t11064: F, t8019: F, t1940: F, t2071: F, t2403: F, t25446: F, t26425: F, t26581: F, t26585: F, t26590: F, t27376: F, t27391: F, t28456: F, t28472: F, t51780: F, t7010: F, t7432: F, t7749: F, t7991: F, t95511: F, t98627: F, t98659: F, t98662: F, t98740: F, t98743: F, t98751: F, t98755: F, t98768: F, t99550: F) -> (F, F) {
    let t103586 = t8019 * t11064;
    let t103612 = F::cast_from(3.0_f64) * t51780 * t7991 - F::cast_from(3.0_f64) * t95511 * t27376 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t26581 * t7749 - t1940 * t26585 * t27391 - F::cast_from(3.0_f64) * t26425 * t98743 + t1940 * t103586 * t25446 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t2071 * t98627 + F::cast_from(6.0_f64) * t26425 * t98768 + F::cast_from(2.0_f64) * t28472 * t99550 - F::cast_from(3.0_f64) * t26425 * t98659 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t2071 * t98751 + t1940 * t26590 * t98740 + F::cast_from(3.0_f64) * t2403 * t28456 * t7010 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t2071 * t98662 - t1940 * t7432 * t98755 / F::cast_from(2.0_f64);
    (t103586, t103612)
}

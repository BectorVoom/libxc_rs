//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2030/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2030(t11064: f64, t8019: f64, t1940: f64, t2071: f64, t2403: f64, t25446: f64, t26425: f64, t26581: f64, t26585: f64, t26590: f64, t27376: f64, t27391: f64, t28456: f64, t28472: f64, t51780: f64, t7010: f64, t7432: f64, t7749: f64, t7991: f64, t95511: f64, t98627: f64, t98659: f64, t98662: f64, t98740: f64, t98743: f64, t98751: f64, t98755: f64, t98768: f64, t99550: f64) -> (f64, f64) {
    let t103586 = t8019 * t11064;
    let t103612 = 3.0_f64 * t51780 * t7991 - 3.0_f64 * t95511 * t27376 + 3.0_f64 / 2.0_f64 * t2403 * t26581 * t7749 - t1940 * t26585 * t27391 - 3.0_f64 * t26425 * t98743 + t1940 * t103586 * t25446 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t98627 + 6.0_f64 * t26425 * t98768 + 2.0_f64 * t28472 * t99550 - 3.0_f64 * t26425 * t98659 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t98751 + t1940 * t26590 * t98740 + 3.0_f64 * t2403 * t28456 * t7010 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t98662 - t1940 * t7432 * t98755 / 2.0_f64;
    (t103586, t103612)
}

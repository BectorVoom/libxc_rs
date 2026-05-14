//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1366/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1366<F: Float>(t127324: F, t28677: F, t22511: F, t33939: F, t4113: F, t19072: F, t27659: F, t35455: F, t19056: F, t25057: F, t123787: F, t28558: F, t112268: F, t123015: F, t123142: F, t123156: F, t123672: F, t127185: F, t14721: F, t19201: F, t231: F, t25070: F, t25077: F, t25112: F, t27506: F, t27642: F, t28552: F, t28567: F, t28579: F, t28671: F, t31386: F, t4635: F, t6035: F, t6045: F, t6242: F, t6243: F, t6249: F, t6250: F, t704: F, t820: F) -> (F, F, F, F) {
    let t127355 = t28677 * t127324;
    let t127359 = t33939 * t22511;
    let t127360 = t4113 * t127359;
    let t127362 = t27659 * t35455 * t19072;
    let t127365 = t25057 * t19056;
    let t127368 = t28558 * t123787;
    let t127382 = -0.17780800291358024692e0 * t28552 * t123672 - 0.10001700163888888889e0 * t28579 * t31386 - 0.10001700163888888889e0 * t6242 * t6045 * t231 * t19201 - 0.97794401602469135806e0 * t6242 * t123015 * t6243 + 0.97794401602469135806e0 * t6249 * t123015 * t6250 + 0.16002720262222222222e1 * t25112 * t27506 * t28671 + 0.1611184118048991131e0 * t127355 + 0.9667104708293946786e0 * t112268 * t127185 - 0.14500657062440920179e1 * t127360 * t127362 + 0.48327307107230638236e1 * t14721 * t127365 - 0.26853068634149852184e-1 * t127368 + 0.33339000546296296298e-1 * t25077 * t6035 * t704 * t4635 * t820 + 0.33339000546296296298e-1 * t28552 * t123156 - 0.22226000364197530865e-1 * t28552 * t123142 + 0.17780800291358024692e0 * t25070 * t27642 * t28567;
    (t127359, t127362, t127365, t127382)
}

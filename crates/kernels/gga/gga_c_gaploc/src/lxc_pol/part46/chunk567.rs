//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 567/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk567<F: Float>(t4130: F, t986: F, t2482: F, t9272: F, t10231: F, t549: F, t544: F, t8410: F, t9562: F, t2365: F, t7906: F, t7025: F, t1429: F, t9550: F, t9554: F, t9557: F, t9560: F, t9564: F, t9569: F, t9571: F, t9575: F, t9577: F, t9579: F, t9582: F, t9584: F) -> (F, F, F) {
    let t10608 = t4130 * t986;
    let t10609 = t10608 * t2482;
    let t10610 = t9272 * t10609;
    let t10611 = 0.57514388930881124514e0 * t10610;
    let t10612 = t549 * t10231;
    let t10615 = t544 * t8410;
    let t10616 = t10615 * t9562;
    let t10617 = 0.44688112439813033337e-1 * t10616;
    let t10618 = t2365 * t7906;
    let t10619 = t7025 * t10618;
    let t10620 = 0.14896037479937677779e-1 * t10619;
    let t10621 = -t10611 + 0.39722766613167140743e-1 * t1429 * t10612 - t10617 + t10620 + t9550 - t9554 + t9557 + t9560 - t9564 - t9569 - t9571 - t9575 + t9577 + t9579 + t9582 - t9584;
    (t10608, t10615, t10621)
}

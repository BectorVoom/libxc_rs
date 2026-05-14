//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1290/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1290<F: Float>(t363: F, t538: F, t3404: F, t422: F, t100519: F, t23711: F, t101504: F, t26692: F, t100588: F, t100678: F, t100905: F, t100932: F, t101279: F, t101282: F, t1013: F, t104637: F, t104671: F, t1643: F, t1651: F, t1736: F, t2030: F, t2071: F, t23701: F, t23705: F, t23715: F, t26695: F, t2983: F, t379: F, t423: F, t5570: F, t920: F, t93169: F, t94434: F) -> (F, F) {
    let t104742 = t363 * t538;
    let t104782 = t422 * t3404;
    let t104788 = 0.26853068634149852184e-1 * t23711 * t100519;
    let t104792 = 0.22226000364197530866e-1 * t26692 * t101504;
    let t104793 = 0.13335600218518518519e0 * t94434 * t93169 * t104637 * t104742 + 0.1611184118048991131e0 * t23701 * t100588 - 0.10741227453659940873e0 * t23701 * t100932 - 0.88904001456790123461e-1 * t23715 * t100678 * t2983 * t104742 + 0.88904001456790123461e-1 * t23705 * t100678 * t2983 * t104671 + 0.33339000546296296298e-1 * t23705 * t5570 * t423 * t920 * t2071 + 0.13335600218518518519e0 * t26692 * t100905 - 0.33339000546296296298e-1 * t23715 * t5570 * t423 * t920 * t2030 + 0.33339000546296296297e-1 * t23705 * t5570 * t26695 * t1651 + 0.44452000728395061729e-1 * t23705 * t5570 * t1736 * t1013 * t1643 - 0.46992870109762241322e0 * t23711 * t101282 + 0.66678001092592592594e-1 * t23705 * t5570 * t104782 * t379 + t104788 - 0.17780800291358024692e0 * t26692 * t101279 + t104792;
    (t104742, t104793)
}

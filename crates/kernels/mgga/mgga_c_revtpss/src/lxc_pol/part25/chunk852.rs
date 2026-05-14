//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 852/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk852<F: Float>(t10836: F, t2662: F, t2661: F, t221: F, t2485: F, t2646: F, t2484: F, t2482: F, t596: F, t823: F, t2487: F, t10794: F, t10799: F, t10803: F, t10807: F, t10812: F, t10816: F, t10820: F, t10824: F, t10826: F, t10828: F, t10833: F, t2745: F, t4362: F, t825: F, t851: F) -> (F, F) {
    let t10837 = t2662 * t10836;
    let t10838 = t2661 * t10837;
    let t10841 = t2485 * t221 * t2646;
    let t10842 = t2484 * t10841;
    let t10845 = t2482 * t823 * t596;
    let t10846 = t10845 * t2487;
    let t10848 = 0.25724410870841842183e-2 * t2745 * t10794 + 0.12862205435420921092e-2 * t4362 * t10799 + 0.25724410870841842183e-2 * t2745 * t10803 - 0.64311027177104605458e-3 * t2745 * t10807 - 0.24009450146119052704e-1 * t10812 - 0.17006693853500995666e-1 * t10816 + 0.12862205435420921092e-1 * t851 * t10820 - t10824 + t10826 - 0.21437009059034868486e-3 * t825 * t10828 - 0.38115002106963996168e-4 * t10833 - 0.17149607247227894789e-3 * t10838 - 0.38115002106963996168e-4 * t10842 + 0.40656002247428262579e-3 * t10846;
    (t10841, t10848)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1578/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1578(t10845: f64, t2487: f64, t10794: f64, t10799: f64, t10803: f64, t10807: f64, t10812: f64, t10816: f64, t10820: f64, t10824: f64, t10826: f64, t10828: f64, t10833: f64, t10838: f64, t10842: f64, t2745: f64, t4362: f64, t825: f64, t851: f64) -> (f64, f64) {
    let t10846 = t10845 * t2487;
    let t10848 = 0.25724410870841842183e-2_f64 * t2745 * t10794 + 0.12862205435420921092e-2_f64 * t4362 * t10799 + 0.25724410870841842183e-2_f64 * t2745 * t10803 - 0.64311027177104605458e-3_f64 * t2745 * t10807 - 0.24009450146119052704e-1_f64 * t10812 - 0.17006693853500995666e-1_f64 * t10816 + 0.12862205435420921092e-1_f64 * t851 * t10820 - t10824 + t10826 - 0.21437009059034868486e-3_f64 * t825 * t10828 - 0.38115002106963996168e-4_f64 * t10833 - 0.17149607247227894789e-3_f64 * t10838 - 0.38115002106963996168e-4_f64 * t10842 + 0.40656002247428262579e-3_f64 * t10846;
    (t10846, t10848)
}

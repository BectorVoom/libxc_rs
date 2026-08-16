//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1970/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1970(t87319: f64, t87320: f64, t92635: f64, t92645: f64, t98744: f64, t98746: f64, t98750: f64, t98752: f64, t98754: f64, t98758: f64, t98762: f64, t98766: f64, t98770: f64, t98774: f64, t98777: f64, t98782: f64, t98787: f64, t98791: f64) -> f64 {
    let t101456 = 0.33913115119077928316e-1_f64 * t98744 + 0.28260929265898273597e-2_f64 * t98746 - t92635 - 0.80745512188280781707e-3_f64 * t98750 + t98752 / 384.0_f64 - t98754 / 384.0_f64 + 0.24223653656484234512e-2_f64 * t98758 - 0.48447307312968469024e-2_f64 * t98762 + 0.16149102437656156341e-2_f64 * t98766 - 0.16956557559538964158e-1_f64 * t98770 - 0.28260929265898273597e-2_f64 * t98774 + t98777 / 768.0_f64 - 0.13457585364713463618e-3_f64 * t98782 + 0.67287926823567318088e-4_f64 * t98787 + 0.67287926823567318088e-4_f64 * t98791 + t87319 - t87320 - t92645;
    t101456
}

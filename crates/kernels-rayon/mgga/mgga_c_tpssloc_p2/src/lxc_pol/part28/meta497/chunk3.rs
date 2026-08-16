//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1718/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1718(t25140: f64, t25144: f64, t23125: f64, t23135: f64, t24230: f64, t24231: f64, t25142: f64, t25147: f64, t25149: f64, t25151: f64, t25156: f64, t23043: f64, t23063: f64, t23071: f64, t23084: f64, t25065: f64, t25069: f64, t25071: f64, t25073: f64, t25107: f64, t25109: f64, t25113: f64, t25117: f64, t25121: f64, t25124: f64, t25126: f64, t25128: f64, t25133: f64, t25136: f64, t26619: f64, t26621: f64, t26630: f64) -> f64 {
    let t26644 = 7.0_f64 / 72.0_f64 * t25140;
    let t26646 = 7.0_f64 / 1152.0_f64 * t25144;
    let t26651 = 0.40372756094140390853e-3_f64 * t23125 + t26644 + 5.0_f64 / 192.0_f64 * t25142 + t26646 - t25147 / 768.0_f64 - t25149 / 768.0_f64 - t25151 / 768.0_f64 + t23135 + t24230 + t24231 + t25156 / 8.0_f64;
    let t26653 = 0.40372756094140390853e-3_f64 * t25065 + t23043 - t25069 / 192.0_f64 - t25071 / 192.0_f64 - t25073 / 192.0_f64 + 0.16956557559538964158e-1_f64 * t23063 + t23071 + t26619 + 0.28260929265898273597e-2_f64 * t23084 - t26621 + t26630 - 0.24223653656484234512e-2_f64 * t25107 + 0.16956557559538964158e-1_f64 * t25109 + 0.24223653656484234512e-2_f64 * t25113 - 0.40372756094140390853e-3_f64 * t25117 + 0.16956557559538964158e-1_f64 * t25121 - 0.40372756094140390853e-3_f64 * t25124 + 0.28260929265898273597e-2_f64 * t25126 - t25128 / 24.0_f64 + 0.67287926823567318088e-4_f64 * t25133 + t25136 / 768.0_f64 + t26651;
    t26653
}

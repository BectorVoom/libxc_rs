//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2021/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2021(t91225: f64, t80780: f64, t80784: f64, t80789: f64, t80792: f64, t80794: f64, t80796: f64, t80801: f64, t80807: f64, t80814: f64, t80821: f64, t80828: f64, t84514: f64, t91229: f64, t91233: f64, t91237: f64, t91241: f64, t91256: f64) -> f64 {
    let t93682 = 0.56521858531796547194e-2_f64 * t91225;
    let t93699 = t93682 - 0.40372756094140390853e-3_f64 * t91229 - 0.48447307312968469024e-2_f64 * t91233 - 0.24223653656484234512e-2_f64 * t91237 + 0.24223653656484234512e-2_f64 * t91241 - 0.12650130242830655801e-1_f64 * t80780 + 0.67287926823567318088e-4_f64 * t80784 + 0.67287926823567318088e-4_f64 * t80789 - 0.21083550404717759668e-2_f64 * t80792 + 119.0_f64 / 1728.0_f64 * t80794 - 7.0_f64 / 1152.0_f64 * t80796 - 0.13457585364713463618e-3_f64 * t80801 + 0.67287926823567318088e-4_f64 * t80807 + 0.40372756094140390853e-3_f64 * t80814 - 7.0_f64 / 144.0_f64 * t80821 - t84514 - 7.0_f64 / 24.0_f64 * t80828 - 0.16956557559538964158e-1_f64 * t91256;
    t93699
}

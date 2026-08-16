//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2107/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2107(t27186: f64, t99404: f64, t98849: f64, t18785: f64, t7053: f64, t92861: f64, t92870: f64, t92873: f64, t92875: f64, t98825: f64, t98830: f64, t98851: f64, t98853: f64, t98856: f64, t98858: f64, t98868: f64) -> f64 {
    let t105960 = t99404 * t27186;
    let t105962 = t98849 * t27186;
    let t105969 = 0.34270468708064099208e-1_f64 * t98825 - 0.14456046980341999104e-1_f64 * t105960 + 0.25702851531048074406e-1_f64 * t105962 + t92861 - t98830 - 0.65854491829355115987e0_f64 * t7053 * t18785 - t92870 - t92873 + t92875 + t98851 + 0.86736281882051994624e-1_f64 * t98853 - t98856 - 0.3427046870806409921e-2_f64 * t98858 - 0.45699670022203476294e-2_f64 * t98868;
    t105969
}

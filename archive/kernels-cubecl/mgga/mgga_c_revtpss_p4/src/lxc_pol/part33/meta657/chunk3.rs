//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2115/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2115<F: Float>(t27186: F, t99404: F, t98849: F, t18785: F, t7053: F, t92861: F, t92870: F, t92873: F, t92875: F, t98825: F, t98830: F, t98851: F, t98853: F, t98856: F, t98858: F, t98868: F) -> F {
    let t105960 = t99404 * t27186;
    let t105962 = t98849 * t27186;
    let t105969 = F::cast_from(0.34270468708064099208e-1_f64) * t98825 - F::cast_from(0.14456046980341999104e-1_f64) * t105960 + F::cast_from(0.25702851531048074406e-1_f64) * t105962 + t92861 - t98830 - F::cast_from(0.65854491829355115987e0_f64) * t7053 * t18785 - t92870 - t92873 + t92875 + t98851 + F::cast_from(0.86736281882051994624e-1_f64) * t98853 - t98856 - F::cast_from(0.3427046870806409921e-2_f64) * t98858 - F::cast_from(0.45699670022203476294e-2_f64) * t98868;
    t105969
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1973/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1973(t84921: f64, t84932: f64, t87437: f64, t87438: f64, t87440: f64, t87445: f64, t92697: f64, t92705: f64, t92710: f64, t92713: f64, t98847: f64, t98849: f64, t98851: f64, t98853: f64, t98858: f64, t98862: f64, t98868: f64, t98871: f64) -> f64 {
    let t101496 = t98847 / 192.0_f64 - 5.0_f64 / 192.0_f64 * t98849 + t98851 / 96.0_f64 - t98853 / 384.0_f64 - t84921 + t87437 - 0.40372756094140390853e-3_f64 * t98858 + 0.24223653656484234512e-2_f64 * t98862 - t87438 - t87440 + t92697 + 0.40372756094140390853e-3_f64 * t87445 - t84932 - t92705 + t98868 / 8.0_f64 + 0.16956557559538964158e-1_f64 * t98871 - t92710 + t92713;
    t101496
}

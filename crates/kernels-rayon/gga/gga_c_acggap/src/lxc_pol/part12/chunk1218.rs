//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1218/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1218(t36289: f64, t36292: f64, t36299: f64, t36302: f64, t36327: f64, t31812: f64, t31816: f64, t31822: f64, t31825: f64, t31832: f64, t36294: f64, t36296: f64, t36306: f64, t36308: f64, t36310: f64, t36314: f64, t36320: f64, t36325: f64) -> f64 {
    let t37940 = 0.37737710747524982482e-2_f64 * t36289;
    let t37941 = 0.21437009059034868486e-2_f64 * t36292;
    let t37944 = 0.28582678745379824648e-2_f64 * t36299;
    let t37945 = 0.17149607247227894789e-2_f64 * t36302;
    let t37957 = 0.18868855373762491241e-1_f64 * t36327;
    let t37958 = -t37940 + t37941 - 0.27953859812981468505e-2_f64 * t36294 + 0.68598428988911579156e-1_f64 * t36296 + t37944 + t37945 - 0.80031500487063509014e-2_f64 * t31812 + 0.40015750243531754507e-2_f64 * t31816 - t36306 / 12.0_f64 - t36308 / 24.0_f64 - t36310 / 24.0_f64 - 0.4584375e-1_f64 * t36314 + 11.0_f64 / 576.0_f64 * t31822 + 0.68598428988911579156e-2_f64 * t31825 - 0.42874018118069736972e-3_f64 * t36320 - 0.21437009059034868486e-2_f64 * t31832 - 0.42874018118069736972e-2_f64 * t36325 - t37957;
    t37958
}

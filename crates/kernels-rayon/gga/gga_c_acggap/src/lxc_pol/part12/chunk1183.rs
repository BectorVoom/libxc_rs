//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1183/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1183(t34957: f64, t34990: f64, t30786: f64, t30790: f64, t34937: f64, t34941: f64, t34945: f64, t34949: f64, t34953: f64, t34961: f64, t34965: f64, t34969: f64, t34973: f64, t34977: f64, t34980: f64, t34984: f64, t34994: f64, t34996: f64) -> f64 {
    let t37311 = 0.57165357490759649296e-3_f64 * t34957;
    let t37321 = 0.57165357490759649296e-3_f64 * t34990;
    let t37324 = 0.12579236915841660828e-2_f64 * t34937 - 0.18868855373762491241e-1_f64 * t34941 - 0.37737710747524982482e-2_f64 * t34945 + 0.85748036236139473944e-3_f64 * t34949 + 0.42874018118069736972e-3_f64 * t34953 + t37311 - 0.62896184579208304138e-2_f64 * t34961 - 0.85748036236139473944e-3_f64 * t34965 + 0.31448092289604152068e-2_f64 * t34969 - 0.12579236915841660828e-2_f64 * t34973 - 0.21437009059034868486e-2_f64 * t34977 + 0.21437009059034868486e-2_f64 * t34980 - 0.12862205435420921092e-1_f64 * t34984 - 0.42874018118069736972e-3_f64 * t30786 - 0.57165357490759649296e-3_f64 * t30790 + t37321 - 0.25724410870841842184e-1_f64 * t34994 - t34996 / 24.0_f64;
    t37324
}

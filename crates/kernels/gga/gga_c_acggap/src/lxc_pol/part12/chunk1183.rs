//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1183/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1183<F: Float>(t34957: F, t34990: F, t30786: F, t30790: F, t34937: F, t34941: F, t34945: F, t34949: F, t34953: F, t34961: F, t34965: F, t34969: F, t34973: F, t34977: F, t34980: F, t34984: F, t34994: F, t34996: F) -> F {
    let t37311 = F::cast_from(0.57165357490759649296e-3_f64) * t34957;
    let t37321 = F::cast_from(0.57165357490759649296e-3_f64) * t34990;
    let t37324 = F::cast_from(0.12579236915841660828e-2_f64) * t34937 - F::cast_from(0.18868855373762491241e-1_f64) * t34941 - F::cast_from(0.37737710747524982482e-2_f64) * t34945 + F::cast_from(0.85748036236139473944e-3_f64) * t34949 + F::cast_from(0.42874018118069736972e-3_f64) * t34953 + t37311 - F::cast_from(0.62896184579208304138e-2_f64) * t34961 - F::cast_from(0.85748036236139473944e-3_f64) * t34965 + F::cast_from(0.31448092289604152068e-2_f64) * t34969 - F::cast_from(0.12579236915841660828e-2_f64) * t34973 - F::cast_from(0.21437009059034868486e-2_f64) * t34977 + F::cast_from(0.21437009059034868486e-2_f64) * t34980 - F::cast_from(0.12862205435420921092e-1_f64) * t34984 - F::cast_from(0.42874018118069736972e-3_f64) * t30786 - F::cast_from(0.57165357490759649296e-3_f64) * t30790 + t37321 - F::cast_from(0.25724410870841842184e-1_f64) * t34994 - t34996 / F::new(24.0);
    t37324
}

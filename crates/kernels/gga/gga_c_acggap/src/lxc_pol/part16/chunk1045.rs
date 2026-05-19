//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1045/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1045<F: Float>(t34804: F, t34844: F, t34879: F, t34897: F, t35022: F, t35043: F, t35055: F, t35076: F, t35180: F, t35204: F, t35238: F, t35240: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37252 = F::cast_from(0.20965394859736101378e-2_f64) * t34804;
    let t37271 = F::cast_from(0.34299214494455789578e-2_f64) * t34844;
    let t37287 = F::cast_from(0.85748036236139473944e-3_f64) * t34879;
    let t37293 = F::cast_from(0.13073958333333333333e0_f64) * t34897;
    let t37345 = F::cast_from(0.57165357490759649296e-3_f64) * t35022;
    let t37363 = F::new(35.0) / F::new(108.0) * t35043;
    let t37366 = F::cast_from(0.15724046144802076034e-2_f64) * t35055;
    let t37375 = F::new(77.0) / F::new(288.0) * t35076;
    let t37426 = F::cast_from(0.21437009059034868486e-3_f64) * t35180;
    let t37435 = F::cast_from(0.13976929906490734252e-1_f64) * t35204;
    let t37446 = F::cast_from(0.21437009059034868486e-2_f64) * t35238;
    let t37447 = F::cast_from(0.12862205435420921092e-1_f64) * t35240;
    (t37252, t37271, t37287, t37293, t37345, t37363, t37366, t37375, t37426, t37435, t37446, t37447)
}

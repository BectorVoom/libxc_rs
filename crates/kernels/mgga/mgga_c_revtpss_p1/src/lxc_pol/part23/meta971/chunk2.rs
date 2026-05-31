//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3281/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3281<F: Float>(t22822: F, t3989: F, t2661: F, t3992: F, t543: F, t86205: F, t1353: F, t1410: F, t1414: F, t221: F, t22852: F, t49071: F, t49093: F, t74638: F, t74641: F, t74656: F, t74660: F, t74664: F, t828: F, t85442: F, t86203: F, t86208: F, t86212: F, t86220: F, t86222: F, t86226: F, t86234: F, t86236: F) -> F {
    let t86240 = t3989 * t22822;
    let t86244 = t2661 * t3992 * t86205 * t543;
    let t86249 = F::cast_from(0.71456696863449561619e-5_f64) * t86203 + F::cast_from(0.42874018118069736973e-4_f64) * t86208 - F::cast_from(0.42874018118069736973e-4_f64) * t86212 - F::cast_from(0.85748036236139473944e-3_f64) * t1410 * t1414 * t828 * t85442 - F::cast_from(0.50820002809285328225e-4_f64) * t86220 + F::cast_from(0.12004725073059526352e0_f64) * t86222 - F::cast_from(0.15246000842785598467e-2_f64) * t86226 - t49071 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t49093 * t221 * t22852 * t1353 + F::cast_from(0.21437009059034868486e-4_f64) * t86234 + F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t86236 - F::cast_from(0.6098400337114239387e-4_f64) * t74638 - F::cast_from(0.13553694749236397037e-4_f64) * t74641 + F::cast_from(0.40015750243531754507e-2_f64) * t86240 + F::cast_from(0.71456696863449561619e-5_f64) * t86244 - F::cast_from(0.24009450146119052704e-1_f64) * t74656 - F::cast_from(0.30492001685571196935e-3_f64) * t74660 + F::cast_from(0.15246000842785598467e-3_f64) * t74664;
    t86249
}

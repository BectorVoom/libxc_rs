//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3127/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3127<F: Float>(t1256: F, t17311: F, t17333: F, t12268: F, t29054: F, t12282: F, t1250: F, t12800: F, t12976: F, t13095: F, t16737: F, t17369: F, t17426: F, t17429: F, t17679: F, t17684: F, t17693: F, t17709: F, t17710: F, t17729: F, t17730: F, t17753: F, t17754: F, t1791: F, t20945: F, t21203: F, t3626: F, t3631: F, t3647: F, t3720: F, t44833: F, t51959: F, t5320: F, t5397: F, t57536: F, t57548: F, t57569: F, t57571: F, t57578: F, t57584: F, t57586: F, t57590: F) -> F {
    let t57602 = t17311 * t1256;
    let t57604 = t17333 * t1256;
    let t57606 = t29054 * t12268;
    let t57610 = F::cast_from(0.71456696863449561621e-3_f64) * t17693 * t20945 * t1250 * t12282 + F::cast_from(0.17149607247227894789e-2_f64) * t17729 * t3626 * t16737 * t17730 + F::cast_from(0.64311027177104605458e-3_f64) * t17753 * t3720 * t57536 * t17754 - F::cast_from(0.57165357490759649295e-3_f64) * t57569 + F::cast_from(0.45732285992607719436e-2_f64) * t57571 * t3631 - F::cast_from(0.85748036236139473944e-3_f64) * t17426 * t17679 + F::cast_from(0.42874018118069736972e-3_f64) * t17429 * t17684 + F::cast_from(0.38586616306262763276e-2_f64) * t17709 * t3720 * t17710 * t57578 + F::cast_from(0.47637797908966374413e-3_f64) * t57584 - F::cast_from(0.85748036236139473944e-3_f64) * t57586 - F::cast_from(0.42874018118069736972e-3_f64) * t57590 - F::cast_from(0.21437009059034868486e-3_f64) * t44833 * t1791 - F::cast_from(0.64311027177104605458e-3_f64) * t12976 * t5320 - F::cast_from(0.42874018118069736972e-3_f64) * t12800 * t5397 - F::cast_from(0.42874018118069736972e-3_f64) * t3647 * t17369 - F::cast_from(0.68598428988911579154e-2_f64) * t21203 * t13095 - F::cast_from(0.22866142996303859718e-2_f64) * t57602 + F::cast_from(0.42874018118069736972e-3_f64) * t57604 + t57548 * t57606 * t51959 / F::cast_from(12.0_f64);
    t57610
}

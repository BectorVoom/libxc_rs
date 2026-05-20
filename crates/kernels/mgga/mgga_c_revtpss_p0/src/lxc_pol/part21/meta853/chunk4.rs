//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3216/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3216<F: Float>(t12916: F, t17704: F, t5340: F, t12898: F, t1804: F, t12948: F, t17529: F, t11262: F, t3711: F, t5278: F, t1042: F, t1122: F, t11231: F, t1214: F, t12832: F, t12912: F, t12931: F, t12933: F, t13312: F, t16775: F, t17212: F, t17505: F, t17552: F, t17649: F, t17729: F, t17736: F, t17744: F, t17750: F, t21035: F, t3584: F, t3626: F, t3647: F, t4186: F, t44521: F, t5051: F, t5296: F, t5405: F, t59391: F, t59401: F, t59404: F, t59406: F, t59408: F, t59411: F) -> F {
    let t59415 = t5340 * t12916 * t17704;
    let t59419 = t1804 * t12898;
    let t59423 = t17529 * t12948;
    let t59426 = t3711 * t11262 * t5278;
    let t59448 = -F::cast_from(0.11433071498151929859e-2_f64) * t59391 - F::cast_from(0.85748036236139473944e-3_f64) * t44521 * t17649 * t17212 * t5405 + F::cast_from(0.85748036236139473944e-3_f64) * t17729 * t3626 * t5051 * t12931 - F::cast_from(0.38586616306262763275e-2_f64) * t59401 * t17750 + F::cast_from(0.30488190661738479624e-2_f64) * t59404 - F::cast_from(0.28582678745379824648e-3_f64) * t59406 - F::cast_from(0.57165357490759649295e-3_f64) * t59408 + F::cast_from(0.12862205435420921092e-2_f64) * t59411 * t12912 + F::cast_from(0.85748036236139473944e-3_f64) * t59415 - F::cast_from(0.64311027177104605458e-3_f64) * t12832 * t17744 - F::cast_from(0.33875767401931644027e-3_f64) * t59419 + F::cast_from(0.42874018118069736973e-2_f64) * t3647 * t17552 + F::cast_from(0.22866142996303859718e-2_f64) * t59423 - F::cast_from(0.95275595817932748825e-4_f64) * t59426 - F::cast_from(0.22866142996303859718e-2_f64) * t17505 * t12933 + F::cast_from(0.42874018118069736972e-3_f64) * t3711 * t1042 * t5296 * t13312 * t1214 + F::cast_from(0.42874018118069736972e-3_f64) * t3711 * t1042 * t5296 * t4186 * t3584 + F::cast_from(0.85748036236139473944e-3_f64) * t17729 * t3626 * t21035 * t11231 - F::cast_from(0.85748036236139473944e-3_f64) * t17736 * t3626 * t16775 * t1122;
    t59448
}

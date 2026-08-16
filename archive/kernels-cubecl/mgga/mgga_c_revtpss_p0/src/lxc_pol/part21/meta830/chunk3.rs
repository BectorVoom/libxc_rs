//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3097/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3097<F: Float>(t12956: F, t17209: F, t3140: F, t5216: F, t3599: F, t3609: F, t1261: F, t17198: F, t3172: F, t1042: F, t12269: F, t12800: F, t12816: F, t12953: F, t13081: F, t17381: F, t17569: F, t17710: F, t17747: F, t17794: F, t17796: F, t21203: F, t3606: F, t3613: F, t3711: F, t3720: F, t44260: F, t44664: F, t5279: F, t5304: F, t53474: F, t5381: F, t56246: F, t56766: F, t56786: F, t56787: F, t56791: F, t56793: F, t56796: F) -> F {
    let t56798 = t12956 * t17209;
    let t56802 = t5216 * t3140;
    let t56803 = t56802 * t3599;
    let t56806 = t56802 * t3609;
    let t56812 = t1261 * t3172 * t17198;
    let t56818 = -F::cast_from(0.38586616306262763275e-2_f64) * t17747 * t3720 * t17710 * t56766 + F::cast_from(0.12862205435420921092e-2_f64) * t44664 * t17381 + F::cast_from(0.14291339372689912324e-2_f64) * t5381 * t12816 - F::cast_from(0.7145669686344956162e-3_f64) * t12956 * t17796 + F::cast_from(0.45732285992607719436e-2_f64) * t21203 * t13081 - F::cast_from(0.14291339372689912324e-2_f64) * t3711 * t1042 * t17794 * t12269 + F::cast_from(0.42874018118069736972e-3_f64) * t17569 * t12953 + t56786 + F::cast_from(0.57165357490759649295e-3_f64) * t56787 + t56791 - F::cast_from(0.1270341277572436651e-2_f64) * t56793 + F::cast_from(0.28582678745379824648e-3_f64) * t56796 + F::cast_from(0.57165357490759649295e-3_f64) * t56798 + F::cast_from(0.42874018118069736972e-3_f64) * t44260 * t5279 + F::cast_from(0.12862205435420921092e-2_f64) * t56803 * t3606 - F::cast_from(0.64311027177104605458e-3_f64) * t56806 * t3613 + F::cast_from(0.71456696863449561621e-3_f64) * t12800 * t5304 - F::cast_from(0.57165357490759649295e-3_f64) * t56812 + F::cast_from(0.85748036236139473944e-2_f64) * t1261 * t1042 * t56246 * t53474;
    t56818
}

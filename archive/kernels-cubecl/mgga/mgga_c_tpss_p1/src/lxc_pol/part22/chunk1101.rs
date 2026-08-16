//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1101/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1101<F: Float>(t1523: F, t2929: F, t1535: F, t2973: F, t11844: F, t11873: F, t11857: F, t11860: F, t11862: F, t11865: F, t11867: F, t11871: F, t11875: F, t11880: F, t11885: F, t11890: F) -> (F, F, F, F) {
    let t12083 = t1523 * t2929;
    let t12086 = t1535 * t2973;
    let t12093 = F::cast_from(0.11038e0_f64) * t11844;
    let t12104 = F::cast_from(0.13418888888888888889e0_f64) * t11873;
    let t12109 = -F::cast_from(0.412621875e-1_f64) * t11857 - F::cast_from(0.258925e1_f64) * t11860 - F::cast_from(0.1294625e1_f64) * t11862 + F::cast_from(0.16504875e0_f64) * t11865 + F::cast_from(0.82524375e-1_f64) * t11867 + F::cast_from(0.49671e0_f64) * t11871 + t12104 - F::cast_from(0.40256666666666666667e0_f64) * t11875 + F::cast_from(0.33547222222222222222e0_f64) * t11880 - F::cast_from(0.12077e1_f64) * t11885 - F::cast_from(0.40256666666666666666e0_f64) * t11890;
    (t12083, t12086, t12093, t12109)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2839/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2839<F: Float>(t2661: F, t2662: F, t4352: F, t6017: F, t23285: F, t2741: F, t14494: F, t14785: F, t14791: F, t14894: F, t1559: F, t23266: F, t2730: F, t2745: F, t36833: F, t40759: F, t40765: F, t40771: F, t50939: F, t50941: F, t61969: F, t61973: F, t61977: F, t61981: F, t61985: F, t62012: F, t62015: F, t76242: F, t76372: F, t76474: F, t775: F, t800: F, t837: F) -> F {
    let t76764 = t2661 * t2662 * t4352 * t6017;
    let t76767 = t2741 * t23285;
    let t76776 = F::cast_from(0.22869001264178397701e-3_f64) * t61969 - F::cast_from(0.85748036236139473944e-4_f64) * t61973 + F::cast_from(0.42874018118069736972e-3_f64) * t61977 - F::cast_from(0.13553694749236397037e-4_f64) * t61981 + F::cast_from(0.15246000842785598468e-3_f64) * t61985 - t40759 + F::cast_from(0.81322168495418382223e-4_f64) * t40765 + t40771 - F::cast_from(0.12862205435420921092e-1_f64) * t2745 * t14785 * t76474 * t837 - F::cast_from(0.38586616306262763276e-2_f64) * t14894 * t36833 * t76242 * t14494 + F::cast_from(0.51448821741683684366e-2_f64) * t2745 * t14791 * t1559 * t76372 + F::cast_from(0.21437009059034868486e-4_f64) * t76764 + F::cast_from(0.91464571985215438873e-3_f64) * t50939 + F::cast_from(0.10003937560882938627e-2_f64) * t76767 + F::new(455.0) / F::new(216.0) * t50941 + t2730 * t800 * t23266 * t775 / F::new(16.0) + F::cast_from(0.27107389498472794075e-4_f64) * t62012 - F::cast_from(0.13553694749236397037e-4_f64) * t62015;
    t76776
}

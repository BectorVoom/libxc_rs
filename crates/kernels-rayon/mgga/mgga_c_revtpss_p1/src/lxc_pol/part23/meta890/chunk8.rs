//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2839/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2839(t2661: f64, t2662: f64, t4352: f64, t6017: f64, t23285: f64, t2741: f64, t14494: f64, t14785: f64, t14791: f64, t14894: f64, t1559: f64, t23266: f64, t2730: f64, t2745: f64, t36833: f64, t40759: f64, t40765: f64, t40771: f64, t50939: f64, t50941: f64, t61969: f64, t61973: f64, t61977: f64, t61981: f64, t61985: f64, t62012: f64, t62015: f64, t76242: f64, t76372: f64, t76474: f64, t775: f64, t800: f64, t837: f64) -> f64 {
    let t76764 = t2661 * t2662 * t4352 * t6017;
    let t76767 = t2741 * t23285;
    let t76776 = 0.22869001264178397701e-3_f64 * t61969 - 0.85748036236139473944e-4_f64 * t61973 + 0.42874018118069736972e-3_f64 * t61977 - 0.13553694749236397037e-4_f64 * t61981 + 0.15246000842785598468e-3_f64 * t61985 - t40759 + 0.81322168495418382223e-4_f64 * t40765 + t40771 - 0.12862205435420921092e-1_f64 * t2745 * t14785 * t76474 * t837 - 0.38586616306262763276e-2_f64 * t14894 * t36833 * t76242 * t14494 + 0.51448821741683684366e-2_f64 * t2745 * t14791 * t1559 * t76372 + 0.21437009059034868486e-4_f64 * t76764 + 0.91464571985215438873e-3_f64 * t50939 + 0.10003937560882938627e-2_f64 * t76767 + 455.0_f64 / 216.0_f64 * t50941 + t2730 * t800 * t23266 * t775 / 16.0_f64 + 0.27107389498472794075e-4_f64 * t62012 - 0.13553694749236397037e-4_f64 * t62015;
    t76776
}

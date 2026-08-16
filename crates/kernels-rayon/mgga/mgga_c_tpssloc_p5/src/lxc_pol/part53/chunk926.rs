//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 926/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk926(t34036: f64, t34066: f64, t113: f64, t1774: f64, t1849: f64, t1983: f64, t2036: f64, t2075: f64, t2096: f64, t33363: f64, t33878: f64, t33883: f64, t33886: f64, t33893: f64, t33900: f64, t33916: f64, t33928: f64, t510: f64, t574: f64, t652: f64, t7685: f64, t7787: f64, t7802: f64, t7890: f64, t7904: f64, t7941: f64, t8329: f64, t8607: f64, t8711: f64, t8718: f64, t8780: f64, t8809: f64, t9003: f64) -> (f64, f64) {
    let t34067 = t34036 + t34066;
    let t34075 = -t8329 + 3.0_f64 * t1983 * t33878 - 4.0_f64 * t9003 * t7802 - 2.0_f64 * t652 * t33883 + 2.0_f64 * t1983 * t33886 + 2.0_f64 * t8607 * t7941 + 2.0_f64 * t33363 * t2096 - 2.0_f64 * t33893 * t510 - 2.0_f64 * t8718 * t1774 - t7685 * t8809 - 2.0_f64 * t1983 * t33900 + 6.0_f64 * t8607 * t7904 + t8780 * t1849 + t33928 * t574 - t113 * t34067 - t33916 * t510 - t8711 * t1774 - 2.0_f64 * t7787 * t2075 - 2.0_f64 * t2036 * t7890;
    (t34067, t34075)
}

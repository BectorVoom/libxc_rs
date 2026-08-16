//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 926/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk926<F: Float>(t34036: F, t34066: F, t113: F, t1774: F, t1849: F, t1983: F, t2036: F, t2075: F, t2096: F, t33363: F, t33878: F, t33883: F, t33886: F, t33893: F, t33900: F, t33916: F, t33928: F, t510: F, t574: F, t652: F, t7685: F, t7787: F, t7802: F, t7890: F, t7904: F, t7941: F, t8329: F, t8607: F, t8711: F, t8718: F, t8780: F, t8809: F, t9003: F) -> (F, F) {
    let t34067 = t34036 + t34066;
    let t34075 = -t8329 + F::cast_from(3.0_f64) * t1983 * t33878 - F::cast_from(4.0_f64) * t9003 * t7802 - F::cast_from(2.0_f64) * t652 * t33883 + F::cast_from(2.0_f64) * t1983 * t33886 + F::cast_from(2.0_f64) * t8607 * t7941 + F::cast_from(2.0_f64) * t33363 * t2096 - F::cast_from(2.0_f64) * t33893 * t510 - F::cast_from(2.0_f64) * t8718 * t1774 - t7685 * t8809 - F::cast_from(2.0_f64) * t1983 * t33900 + F::cast_from(6.0_f64) * t8607 * t7904 + t8780 * t1849 + t33928 * t574 - t113 * t34067 - t33916 * t510 - t8711 * t1774 - F::cast_from(2.0_f64) * t7787 * t2075 - F::cast_from(2.0_f64) * t2036 * t7890;
    (t34067, t34075)
}

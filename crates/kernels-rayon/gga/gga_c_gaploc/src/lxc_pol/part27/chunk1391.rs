//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1391/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1391(t12089: f64, t1445: f64, t1457: f64, t1572: f64, t1584: f64, t30735: f64, t30751: f64, t30754: f64, t30757: f64, t30760: f64, t30768: f64, t34531: f64, t34533: f64, t34535: f64, t34536: f64, t3714: f64, t38299: f64, t38313: f64, t38393: f64, t4425: f64, t4598: f64, t568: f64, t569: f64, t574: f64, t597: f64) -> f64 {
    let t38607 = 0.14300195980740170668e1_f64 * t1572 * t1457 * t38393 - 0.46011511144704899612e1_f64 * t1584 * t1445 * t38299 + 0.1022478025437886658e1_f64 * t597 * t4598 * t3714 - 0.23005755572352449806e1_f64 * t574 * t568 * t569 * t38313 - t34531 - t30735 - 0.51123901271894332905e0_f64 * t4425 * t12089 - t34533 - 0.10224780254378866581e1_f64 * t30751 + t30754 + t30757 + t30760 + t34535 - t34536 - t30768;
    t38607
}

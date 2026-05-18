//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1391/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1391<F: Float>(t12089: F, t1445: F, t1457: F, t1572: F, t1584: F, t30735: F, t30751: F, t30754: F, t30757: F, t30760: F, t30768: F, t34531: F, t34533: F, t34535: F, t34536: F, t3714: F, t38299: F, t38313: F, t38393: F, t4425: F, t4598: F, t568: F, t569: F, t574: F, t597: F) -> F {
    let t38607 = F::new(0.14300195980740170668e1) * t1572 * t1457 * t38393 - F::new(0.46011511144704899612e1) * t1584 * t1445 * t38299 + F::new(0.1022478025437886658e1) * t597 * t4598 * t3714 - F::new(0.23005755572352449806e1) * t574 * t568 * t569 * t38313 - t34531 - t30735 - F::new(0.51123901271894332905e0) * t4425 * t12089 - t34533 - F::new(0.10224780254378866581e1) * t30751 + t30754 + t30757 + t30760 + t34535 - t34536 - t30768;
    t38607
}

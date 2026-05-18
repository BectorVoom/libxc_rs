//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1395/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1395<F: Float>(t38276: F, t447: F, t12001: F, t12135: F, t1445: F, t1450: F, t1589: F, t204: F, t2476: F, t30920: F, t34773: F, t34774: F, t34775: F, t34776: F, t34783: F, t34790: F, t34794: F, t34797: F, t34801: F, t38392: F, t38393: F, t4771: F, t557: F, t597: F) -> (F, F) {
    let t38731 = t38276 * t447;
    let t38738 = -F::new(0.46011511144704899612e1) * t4771 * t12135 - F::new(0.46011511144704899612e1) * t1450 * t1445 * t38392 * t447 + F::new(0.23005755572352449806e2) * t597 * t1445 * t38393 - t34773 - t34774 + t30920 - t34775 - t34776 + F::new(0.92023022289409799224e1) * t2476 * t204 * t38731 + t34783 + t34790 - t34794 - t34797 - t34801 - F::new(0.47667319935800568892e0) * t557 * t1589 * t12001;
    (t38731, t38738)
}

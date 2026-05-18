//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1400/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1400<F: Float>(t10513: F, t20441: F, t6914: F, t10532: F, t4529: F, t579: F, t30903: F, t30907: F, t30923: F, t30927: F, t10417: F, t1397: F) -> (F, F, F, F, F, F, F) {
    let t34762 = F::new(0.1656414401209376386e3) * t6914 * t20441 * t10513;
    let t34766 = F::new(0.73618417831527839379e2) * t10532 * t579 * t4529 * t10513;
    let t34773 = F::new(0.63904876589867916128e-1) * t30903;
    let t34774 = F::new(0.95857314884801874192e-1) * t30907;
    let t34775 = F::new(0.31952438294933958064e-1) * t30923;
    let t34776 = F::new(0.12780975317973583226e0) * t30927;
    let t34777 = t1397 * t10417;
    (t34762, t34766, t34773, t34774, t34775, t34776, t34777)
}

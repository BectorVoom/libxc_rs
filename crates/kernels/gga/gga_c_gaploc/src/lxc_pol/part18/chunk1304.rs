//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1304/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1304<F: Float>(t1445: F, t2087: F, t24926: F, t935: F, t10820: F, t10914: F, t2089: F, t539: F, t16036: F, t6111: F, t2028: F, t28593: F, t28633: F, t28636: F, t33376: F, t33381: F, t33385: F, t33387: F, t33389: F, t33392: F, t33394: F, t33397: F, t33399: F) -> F {
    let t33405 = F::new(0.69017266717057349418e1) * t2087 * t1445 * t24926 * t935;
    let t33409 = F::new(0.28600391961480341335e1) * t10914 * t539 * t2089 * t10820;
    let t33412 = F::new(0.57200783922960682671e1) * t6111 * t16036 * t10820;
    let t33413 = t28593 + t33376 + t33381 - t33385 + t33387 - t33389 + t33392 + t33394 - t33397 - F::new(0.79445533226334281486e-1) * t33399 * t2028 - t33405 - t33409 + t33412 - t28633 + t28636;
    t33413
}

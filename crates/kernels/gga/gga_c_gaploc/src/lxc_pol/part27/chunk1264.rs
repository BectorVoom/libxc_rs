//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1264/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1264<F: Float>(t33388: F, t1980: F, t8788: F, t9824: F, t22424: F, t3500: F, t2975: F, t6134: F, t7372: F, t1445: F, t2087: F, t24926: F, t935: F) -> (F, F, F, F, F) {
    let t33389 = F::new(0.85206502119823888168e-1) * t33388;
    let t33391 = t1980 * t8788 * t9824;
    let t33392 = F::new(0.29792074959875355558e-1) * t33391;
    let t33393 = t22424 * t3500;
    let t33394 = F::new(0.19171462976960374838e0) * t33393;
    let t33396 = t6134 * t2975 * t7372;
    let t33397 = F::new(0.29792074959875355558e-1) * t33396;
    let t33405 = F::new(0.69017266717057349418e1) * t2087 * t1445 * t24926 * t935;
    (t33389, t33392, t33394, t33397, t33405)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1045/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1045<F: Float>(t1016: F, t1382: F, t7058: F, t2761: F, t6295: F, t6525: F, t10163: F, t29874: F, t2321: F, t26673: F, t9074: F, t26629: F, t10141: F, t6313: F, t10145: F, t6305: F) -> (F, F, F, F, F, F, F) {
    let t31483 = 2.0 * t1382 * t1016 * t7058;
    let t31487 = t6525 * t2761 * t6295;
    let t31488 = 0.11856252764865062333e-2 * t31487;
    let t31489 = t29874 * t10163;
    let t31490 = 0.23712505529730124666e-2 * t31489;
    let t31492 = t9074 * t26673 * t2321;
    let t31493 = 0.11856252764865062333e-2 * t31492;
    let t31495 = t9074 * t26629 * t2321;
    let t31496 = 0.23712505529730124666e-2 * t31495;
    let t31498 = 0.15176003539027279786e0 * t6313 * t10141;
    let t31522 = 0.1138200265427045984e0 * t6305 * t10145;
    (t31483, t31488, t31490, t31493, t31496, t31498, t31522)
}

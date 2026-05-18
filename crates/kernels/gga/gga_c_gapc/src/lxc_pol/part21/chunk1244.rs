//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1244/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1244<F: Float>(t3640: F, t8341: F, t11224: F, t2933: F, t1484: F, t11203: F, t8286: F, t8297: F, t11254: F, t518: F, t1460: F, t3652: F) -> (F, F, F, F, F, F) {
    let t35386 = t8341 * t3640;
    let t35388 = t2933 * t11224;
    let t35390 = t1484 * t3640;
    let t35393 = t8286 * t11203 * t8297;
    let t35395 = t518 * t11254;
    let t35397 = t1460 * t3652;
    (t35386, t35388, t35390, t35393, t35395, t35397)
}

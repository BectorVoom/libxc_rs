//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1093/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1093<F: Float>(t105416: F, t1359: F, t9132: F, t40465: F, t27083: F, t95053: F, t2112: F, t24: F, t23649: F, t27065: F, t27069: F, t1570: F, t6615: F, t6674: F, t8232: F, t26768: F, t358: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t105417 = t105416 / 3.0;
    let t105425 = t9132 * t1359;
    let t105429 = t40465 * t1359;
    let t105433 = t95053 * t27083;
    let t105434 = t105433 / 18.0;
    let t105435 = t24 * t2112;
    let t105457 = t23649 * t27065;
    let t105458 = 2.0 / 9.0 * t105457;
    let t105459 = t23649 * t27069;
    let t105460 = 2.0 / 9.0 * t105459;
    let t105462 = t6615 * t1570;
    let t105467 = t8232 * t6674;
    let t105473 = t26768 * t358;
    (t105417, t105425, t105429, t105433, t105434, t105435, t105457, t105458, t105459, t105460, t105462, t105467, t105473)
}

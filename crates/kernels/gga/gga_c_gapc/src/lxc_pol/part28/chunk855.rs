//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 855/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk855<F: Float>(t11210: F, t1649: F, t11208: F, t3707: F, t6: F, t101: F, t4050: F, t4055: F, t520: F, t2933: F, t3640: F, t125: F, t505: F, t200: F, t1954: F, t1006: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11211 = t11210 * t1649;
    let t11212 = t11208 * t11211;
    let t11214 = t6 * t3707;
    let t11215 = t11214 * t101;
    let t11216 = t11215 * t4050;
    let t11217 = t520 * t4055;
    let t11218 = t11216 * t11217;
    let t11220 = t2933 * t3640;
    let t11222 = t125 * t505;
    let t11223 = t11222 * t200;
    let t11224 = t11223 * t1954;
    let t11225 = t1006 * t11224;
    (t11211, t11212, t11214, t11215, t11216, t11217, t11218, t11220, t11223, t11224, t11225)
}

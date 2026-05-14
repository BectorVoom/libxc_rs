//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 790/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk790<F: Float>(t1457: F, t8000: F, t8004: F, t1305: F, t2787: F, t1445: F, t1555: F, t999: F, t2822: F, t528: F, t1564: F, t2754: F, t475: F, t7980: F, t1265: F, t2778: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8077 = t1457 * t8000;
    let t8080 = t1457 * t8004;
    let t8083 = t2787 * t1305;
    let t8084 = t1445 * t8083;
    let t8087 = t1555 * t999;
    let t8090 = t528 * t2822;
    let t8097 = t1564 * t2754;
    let t8098 = t8097 * t475;
    let t8099 = t1445 * t8098;
    let t8104 = t7980 * t475;
    let t8105 = t1445 * t8104;
    let t8108 = t2778 * t1265;
    (t8077, t8080, t8084, t8087, t8090, t8097, t8099, t8105, t8108)
}
